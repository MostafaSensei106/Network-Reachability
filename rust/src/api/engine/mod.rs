//! The core orchestration engine for network checks.

pub mod sampler;
pub mod security;

use crate::api::{
    analysis::{compute_latency_stats, evaluate_network_quality},
    models::{ConnectionQuality, NetworkConfiguration, NetworkReport, NetworkStatus},
    probes::{self, detect_security_and_network_type},
};
use ::chrono::Utc;

use sampler::collect_network_samples;
use security::perform_dns_security_check;

/// The main entry point for running a comprehensive network check.
///
/// This function orchestrates the entire check process:
/// 1. Collects latency samples via [sampler::collect_network_samples].
/// 2. Computes statistics via [analysis::compute_latency_stats].
/// 3. Evaluates quality via [analysis::evaluate_network_quality].
/// 4. Detects interface security and type.
/// 5. Compiles a final [NetworkReport].
pub async fn check_network(config: NetworkConfiguration) -> NetworkReport {
    let start_time = Utc::now().timestamp_millis() as u64;

    let (all_sample_latencies, final_target_reports) = collect_network_samples(&config).await;

    let is_connected = !all_sample_latencies.is_empty();
    let num_samples = std::cmp::max(1, config.resilience.num_jitter_samples);

    let latency_stats = compute_latency_stats(
        &all_sample_latencies,
        num_samples,
        &config.quality_threshold,
    );

    let mut quality = evaluate_network_quality(is_connected, &latency_stats, &config);

    // If we're ostensibly connected, check for a captive portal to be sure.
    if is_connected && quality != ConnectionQuality::Offline {
        let cp_status = probes::check_for_captive_portal(1000).await;
        if cp_status.is_captive_portal {
            quality = ConnectionQuality::CaptivePortal;
        }
    }

    let (mut security_flags_res, connection_type) = detect_security_and_network_type();
    perform_dns_security_check(&config, &mut security_flags_res).await;

    let winner_target = if let Some(r) = final_target_reports.iter().find(|r| r.success) {
        r.label.clone()
    } else {
        String::new()
    };

    NetworkReport {
        timestamp_ms: start_time,
        status: NetworkStatus {
            is_connected,
            quality,
            latency_stats,
            winner_target,
        },
        connection_type,
        security_flags_result: security_flags_res,
        target_reports: final_target_reports,
    }
}
