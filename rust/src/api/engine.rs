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

        if sample_num == num_samples - 1 {
            final_reports = reports;
        }
    }

    (all_sample_latencies, final_reports)
}

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
            if let Some(lat) = report.latency_ms {
                if lat < best_latency {
                    best_latency = lat;
                }
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

fn compute_latency_stats(latencies: &[u64], total_expected_samples: u8) -> LatencyStats {
    let packet_loss_percent = if total_expected_samples > 0 {
        let successful_samples = latencies.len() as f32;
        100.0 * (1.0 - (successful_samples / total_expected_samples as f32))
    } else {
        0.0
    };

    let (min_lat, max_lat, mean_lat, std_dev_lat) = calculate_jitter_stats(latencies);

    let final_latency = mean_lat.unwrap_or(0);
    let final_jitter = std_dev_lat.unwrap_or(0.0);

    let latency_stability_score = (100.0 - (final_jitter * 2.0)).clamp(0.0, 100.0);

    let jitter_stability_score = if latencies.len() > 1 {
        let mut jitter_sum = 0.0;
        for i in 1..latencies.len() {
            jitter_sum += (latencies[i] as f64 - latencies[i - 1] as f64).abs();
        }

        let jitter_avg = jitter_sum / (latencies.len() - 1) as f64;

        (100.0 - (jitter_avg * 3.0)).clamp(0.0, 100.0)
    } else {
        if packet_loss_percent > 0.0 {
            50.0
        } else {
            100.0
        }
    };

    let loss_score = (100.0 - (packet_loss_percent as f64 * 2.0)).clamp(0.0, 100.0);

    // Latency Variation (40%) + Sequential Jitter (30%) + Packet Loss (30%)
    let weighted_score =
        (latency_stability_score * 0.4) + (jitter_stability_score * 0.3) + (loss_score * 0.3);

    // ---------------------------------------------------------

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

fn evaluate_network_quality(
    is_connected: bool,
    stats: &LatencyStats,
    config: &NetworkConfiguration,
) -> ConnectionQuality {
    if !is_connected {
        return ConnectionQuality::Offline;
    }

    let quality = evaluate_quality(stats.latency_ms, &config.quality_threshold);

    if stats.stability_score < config.resilience.stability_thershold {
        return ConnectionQuality::Unstable;
    }

    if stats.packet_loss_percent > config.resilience.critical_packet_loss_precent {
        return ConnectionQuality::Unstable;
    }

    return quality;
}

async fn perform_dns_security_check(config: &NetworkConfiguration, flags: &mut SecurityFlags) {
    if !config.security.detect_dns_hijack {
        return;
    }

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

fn get_winner_target(reports: &[TargetReport]) -> String {
    reports
        .iter()
        .find(|r| r.success)
        .map_or_else(String::new, |r| r.label.clone())
}

/// The main entry point for running a comprehensive network check.
///
/// This function orchestrates the various probes and analyses based on the provided configuration.
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

    return NetworkReport {
        timestamp_ms: start_time,
        status: NetworkStatus {
            is_connected: is_connected,
            quality: quality,
            latency_stats: latency_stats,
            winner_target: winner_target,
        },
        connection_type: connection_type,
        security_flags: security_flags,
        target_reports: final_target_reports,
    };
}
