use ::chrono::Utc;
use ::futures::future::join_all;

use crate::api::{
    analysis::{calculate_jitter_stats, evaluate_quality},
    models::{
        CheckStrategy, ConnectionQuality, NetworkConfiguration, NetworkReport, NetworkStatus,
    },
    probes::{self, check_target, detect_security_and_network_type},
};

/// The main entry point for running a comprehensive network check.
///
/// This function orchestrates the various probes and analyses based on the provided configuration.
pub async fn check_network(config: NetworkConfiguration) -> NetworkReport {
    let start_time = Utc::now().timestamp_millis() as u64;

    let mut all_sample_latencies: Vec<u64> = Vec::new();
    let mut final_reports = Vec::new();

    let num_samples = if config.resilience.num_jitter_samples > 1 {
        config.resilience.num_jitter_samples
    } else {
        1
    };

    for sample_num in 0..num_samples {
        let futures = config.targets.iter().map(check_target);
        let reports = join_all(futures).await;

        let mut best_latency_in_sample = u64::MAX;
        let mut success_count_in_sample = 0;
        let mut essential_failed_in_sample = false;

        for report in &reports {
            if report.is_essential && !report.success {
                essential_failed_in_sample = true;
            }

            if report.success {
                success_count_in_sample += 1;
                if let Some(lat) = report.latency_ms {
                    if lat < best_latency_in_sample {
                        best_latency_in_sample = lat;
                    }
                }
            }
        }

        let is_connected_in_sample = if essential_failed_in_sample {
            false
        } else {
            match config.resilience.strategy {
                CheckStrategy::Race => success_count_in_sample > 0,
                CheckStrategy::Consensus => {
                    let total = config.targets.len();
                    success_count_in_sample >= (total / 2) + 1
                }
            }
        };

        if is_connected_in_sample {
            all_sample_latencies.push(best_latency_in_sample);
        }

        // Only store the reports from the last sample run.
        if sample_num == num_samples - 1 {
            final_reports = reports;
        }
    }

    let is_connected = !all_sample_latencies.is_empty();

    let packet_loss_percent = if num_samples > 0 {
        let successful_samples = all_sample_latencies.len() as f32;
        100.0 * (1.0 - (successful_samples / num_samples as f32))
    } else {
        0.0
    };

    let (min_lat, max_lat, mean_lat, std_dev_lat) = calculate_jitter_stats(&all_sample_latencies);
    let final_latency = mean_lat.unwrap_or(0);
    let final_jitter = std_dev_lat.unwrap_or(0.0) as u64;

    let mut quality = if is_connected {
        evaluate_quality(final_latency, &config.quality_threshold)
    } else {
        ConnectionQuality::Dead
    };

    // Check for unstable quality if jitter analysis was performed.
    if num_samples > 1 && final_latency > 0 {
        let jitter_threshold = final_latency as f64 * config.resilience.jitter_threshold_percent;
        if final_jitter as f64 > jitter_threshold {
            quality = ConnectionQuality::Unstable;
        }
    }

    let (mut security_flags, conn_type) = detect_security_and_network_type();

    // --- Security Checks ---
    if config.security.detect_dns_hijack {
        // Use the first essential target, or the first target overall, as a heuristic for the check.
        if let Some(target) = config
            .targets
            .iter()
            .find(|t| t.is_essential)
            .or_else(|| config.targets.first())
        {
            if probes::detect_dns_hijacking(&target.host).await {
                security_flags.is_dns_spoofed = true;
            }
        }
    }

    NetworkReport {
        timestamp_ms: start_time,
        status: NetworkStatus {
            is_connected,
            quality,
            latency_ms: final_latency,
            jitter_ms: final_jitter,
            packet_loss_percent,
            winner_target: if is_connected {
                final_reports
                    .iter()
                    .find(|r| r.success)
                    .map_or_else(String::new, |r| r.label.clone())
            } else {
                String::new()
            },
            min_latency_ms: min_lat,
            max_latency_ms: max_lat,
        },
        connection_type: conn_type,
        security_flags,
        target_reports: final_reports,
    }
}
