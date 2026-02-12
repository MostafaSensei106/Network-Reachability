//! The core orchestration engine for network checks.

use ::chrono::Utc;
use ::futures::future::join_all;

use crate::api::{
    analysis::{calculate_jitter_stats, evaluate_quality},
    models::{
        CheckStrategy, ConnectionQuality, LatencyStats, NetworkConfiguration, NetworkReport,
        NetworkStatus, SecurityFlags, TargetReport,
    },
    probes::{self, check_target, detect_security_and_network_type},
};

/// Collects multiple latency samples by running checks against all configured targets.
async fn collect_network_samples(config: &NetworkConfiguration) -> (Vec<u64>, Vec<TargetReport>) {
    let mut all_sample_latencies = Vec::new();
    let mut final_reports = Vec::new();

    let num_samples = if config.resilience.num_jitter_samples > 1 {
        config.resilience.num_jitter_samples
    } else {
        1
    };

    for sample_num in 0..num_samples {
        let futures = config.targets.iter().map(check_target);
        let reports = join_all(futures).await;

        if let Some(best_latency) = analyze_single_sample(&reports, config) {
            all_sample_latencies.push(best_latency);
        }

        // Only the reports from the very last sample run are stored.
        if sample_num == num_samples - 1 {
            final_reports = reports;
        }
    }

    (all_sample_latencies, final_reports)
}

/// Analyzes the results of a single sample run across all targets.
fn analyze_single_sample(reports: &[TargetReport], config: &NetworkConfiguration) -> Option<u64> {
    let mut best_latency = u64::MAX;
    let mut success_count = 0;
    let mut essential_failed = false;

    for report in reports {
        if report.is_essential && !report.success {
            essential_failed = true;
        }

        if report.success {
            success_count += 1;
            let lat = report.latency_ms;
            if lat < best_latency {
                best_latency = lat;
            }
        }
    }

    if essential_failed {
        return None;
    }

    let is_sample_successful = match config.resilience.strategy {
        CheckStrategy::Race => success_count > 0,
        CheckStrategy::Consensus => {
            let total = config.targets.len();
            success_count >= (total / 2) + 1
        }
    };

    if is_sample_successful {
        Some(best_latency)
    } else {
        None
    }
}

/// Computes final latency and stability statistics from a set of samples.
fn compute_latency_stats(latencies: &[u64], total_expected_samples: u8) -> LatencyStats {
    let successful_samples = latencies.len() as f32;

    let packet_loss_percent = if total_expected_samples > 0 {
        100.0 * (1.0 - (successful_samples / total_expected_samples as f32))
    } else {
        0.0
    };

    let (min_lat, max_lat, mean_lat, std_dev_lat) = calculate_jitter_stats(latencies);

    let final_latency = mean_lat.unwrap_or(0);
    let final_jitter = std_dev_lat.unwrap_or(0.0);
    let mean_latency_f64 = final_latency as f64;

    // ---------------------------------------------------------
    //  1. Calculate Latency Stability based on Coefficient of Variation (CV)
    // ---------------------------------------------------------
    let latency_stability_score = if mean_latency_f64 > 0.0 {
        // CV = Jitter / Mean
        // If the jitter is high (0.25 or more), the stability score decreases
        let cv = final_jitter / mean_latency_f64;

        // Formula: 100 * (1 - (CV * 3.0))
        // If the CV is high, the stability score will be low
        // If the CV is low, the stability score will be high
        (100.0 * (1.0 - (cv * 3.0))).clamp(0.0, 100.0)
    } else {
        // If there is no jitter, the stability score will be 100
        // If there is packet loss, the stability score will be 0
        if packet_loss_percent > 0.0 {
            0.0
        } else {
            100.0
        }
    };
    // ---------------------------------------------------------
    //  2. Calculate Sequential Jitter (SJ)
    // ---------------------------------------------------------
    let jitter_stability_score = if latencies.len() > 1 && mean_latency_f64 > 0.0 {
        let mut jitter_sum = 0.0;
        for i in 1..latencies.len() {
            jitter_sum += (latencies[i] as f64 - latencies[i - 1] as f64).abs();
        }
        let avg_seq_jitter = jitter_sum / (latencies.len() - 1) as f64;

        // Normalize the Sequential Jitter by the mean latency
        let relative_seq_jitter = avg_seq_jitter / mean_latency_f64;

        // Scale the relative Sequential Jitter to a score between 0 and 100
        (100.0 * (1.0 - (relative_seq_jitter * 4.0))).clamp(0.0, 100.0)
    } else {
        100.0
    };

    // ---------------------------------------------------------
    //  3. Calculate Loss Score
    // ---------------------------------------------------------
    // Any loss above 5% will penalize the score
    // The loss score will be 50% if the loss is 5%
    let loss_score = if packet_loss_percent > 0.0 {
        (100.0 - (packet_loss_percent as f64 * 10.0)).clamp(0.0, 100.0)
    } else {
        100.0
    };

    // ---------------------------------------------------------
    //  4. Calculate Spike Penalty
    // ---------------------------------------------------------
    // If the maximum latency is more than twice the mean latency, apply a spike penalty
    let spike_score = if let (Some(max), true) = (max_lat, mean_latency_f64 > 0.0) {
        let ratio = max as f64 / mean_latency_f64;
        if ratio > 2.0 {
            // If the spike is more than twice the mean latency, penalize by 20 degrees
            80.0
        } else {
            100.0
        }
    } else {
        100.0
    };

    // Combine the scores using a weighted average
    let weighted_score = (latency_stability_score * 0.35)
        + (jitter_stability_score * 0.25)
        + (loss_score * 0.30)
        + (spike_score * 0.10); // 10% is reserved for spike penalties only

    LatencyStats {
        latency_ms: final_latency,
        jitter_ms: final_jitter as u64,
        packet_loss_percent,
        min_latency_ms: min_lat,
        max_latency_ms: max_lat,
        avg_latency_ms: mean_lat,
        stability_score: weighted_score as u8,
    }
}

/// Evaluates the final network quality based on latency, stability, and packet loss.
fn evaluate_network_quality(
    is_connected: bool,
    stats: &LatencyStats,
    config: &NetworkConfiguration,
) -> ConnectionQuality {
    if !is_connected {
        return ConnectionQuality::Offline;
    }

    // First, determine quality based purely on latency
    let quality_based_on_speed = evaluate_quality(stats.latency_ms, &config.quality_threshold);

    // Then, apply penalties that can downgrade the quality
    if stats.packet_loss_percent > config.resilience.critical_packet_loss_precent {
        return ConnectionQuality::Unstable;
    }

    if stats.stability_score < config.resilience.stability_thershold {
        // If stability is poor, the best it can be is 'Good'
        return ConnectionQuality::Good;
    }

    // An 'Excellent' connection requires high stability
    if quality_based_on_speed == ConnectionQuality::Excellent && stats.stability_score < 85 {
        return ConnectionQuality::Good;
    }

    quality_based_on_speed
}

/// Runs the DNS hijack check if enabled in the configuration.
async fn perform_dns_security_check(config: &NetworkConfiguration, flags: &mut SecurityFlags) {
    if !config.security.detect_dns_hijack {
        return;
    }

    // Use an essential target if available, otherwise the first one, for the check.
    let target_to_check = config
        .targets
        .iter()
        .find(|t| t.is_essential)
        .or_else(|| config.targets.first());

    if let Some(target) = target_to_check {
        if probes::detect_dns_hijacking(&target.host).await {
            flags.is_dns_spoofed = true;
        }
    }
}

/// Gets the label of the first successful target from a list of reports.
fn get_winner_target(reports: &[TargetReport]) -> String {
    reports
        .iter()
        .find(|r| r.success)
        .map_or_else(String::new, |r| r.label.clone())
}

/// The main entry point for running a comprehensive network check.
///
/// This function orchestrates the entire check process:
/// 1. Records the start time.
/// 2. Collects multiple latency samples by running probes against all configured targets.
/// 3. Computes detailed latency and stability statistics from the samples.
/// 4. Evaluates the final connection quality based on speed, jitter, and packet loss.
/// 5. Detects the network interface type and checks for security issues like VPNs.
/// 6. Performs a DNS hijacking check if configured.
/// 7. Compiles all results into a single, comprehensive [NetworkReport].
///
/// # Arguments
///
/// * `config` - The [NetworkConfiguration] that defines how the check should be performed.
///
/// # Returns
///
/// A [NetworkReport] containing the complete results of the check.
pub async fn check_network(config: NetworkConfiguration) -> NetworkReport {
    let start_time = Utc::now().timestamp_millis() as u64;

    let (all_sample_latencies, final_target_reports) = collect_network_samples(&config).await;

    let is_connected = !all_sample_latencies.is_empty();
    let num_samples = std::cmp::max(1, config.resilience.num_jitter_samples);

    let latency_stats = compute_latency_stats(&all_sample_latencies, num_samples);

    let quality = evaluate_network_quality(is_connected, &latency_stats, &config);

    let (mut security_flags, connection_type) = detect_security_and_network_type();
    perform_dns_security_check(&config, &mut security_flags).await;

    let winner_target = get_winner_target(&final_target_reports);

    NetworkReport {
        timestamp_ms: start_time,
        status: NetworkStatus {
            is_connected,
            quality,
            latency_stats,
            winner_target,
        },
        connection_type,
        security_flags,
        target_reports: final_target_reports,
    }
}
