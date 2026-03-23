//! Sampling and result aggregation logic for network probes.

use crate::api::{
    models::{CheckStrategy, NetworkConfiguration, TargetReport},
    probes::check_target,
};
use futures::future::join_all;

/// Collects multiple latency samples by running checks against all configured targets.
pub async fn collect_network_samples(
    config: &NetworkConfiguration,
) -> (Vec<u64>, Vec<TargetReport>) {
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
            final_reports = reports
        }
    }

    (all_sample_latencies, final_reports)
}

/// Analyzes the results of a single sample run across all targets.
pub fn analyze_single_sample(
    reports: &[TargetReport],
    config: &NetworkConfiguration,
) -> Option<u64> {
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

    // If any essential target fails, the entire sample is invalid.
    if essential_failed {
        return None;
    }

    let is_sample_successful = match config.resilience.strategy {
        CheckStrategy::Race => success_count > 0,
        CheckStrategy::Consensus => {
            let total = config.targets.len();
            success_count >= (total / 2)
        }
    };

    if is_sample_successful {
        Some(best_latency)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::{NetworkConfiguration, TargetReport};

    #[test]
    fn test_analyze_single_sample_logic() {
        let config = NetworkConfiguration::default();
        let reports = vec![
            TargetReport {
                label: "A".into(),
                success: true,
                latency_ms: 100,
                error: None,
                is_essential: false,
            },
            TargetReport {
                label: "B".into(),
                success: false,
                latency_ms: 0,
                error: Some("fail".into()),
                is_essential: false,
            },
        ];

        // Race strategy: one success is enough
        let res = analyze_single_sample(&reports, &config);
        assert_eq!(res, Some(100));

        // Essential failed
        let reports_essential_fail = vec![
            TargetReport {
                label: "A".into(),
                success: true,
                latency_ms: 100,
                error: None,
                is_essential: false,
            },
            TargetReport {
                label: "B".into(),
                success: false,
                latency_ms: 0,
                error: Some("fail".into()),
                is_essential: true,
            },
        ];
        assert_eq!(
            analyze_single_sample(&reports_essential_fail, &config),
            None
        );
    }
}
