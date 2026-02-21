//! Functions for analyzing latency data to determine connection quality and stability.

use crate::api::models::{
    ConnectionQuality, LatencyStats, NetworkConfiguration, QualityThresholds,
};
pub use super::stats::calculate_jitter_stats;

/// Evaluates connection quality based on a latency value and a set of thresholds.
///
/// # Arguments
///
/// * `latency` - The latency value in milliseconds.
/// * `threshold` - A reference to the [QualityThresholds] to compare against.
///
/// # Returns
///
/// A [ConnectionQuality] enum variant representing the quality level.
pub fn evaluate_quality(latency: u64, threshold: &QualityThresholds) -> ConnectionQuality {
    if latency <= threshold.excellent {
        ConnectionQuality::Excellent
    } else if latency <= threshold.great {
        ConnectionQuality::Great
    } else if latency <= threshold.good {
        ConnectionQuality::Good
    } else if latency <= threshold.moderate {
        ConnectionQuality::Moderate
    } else if latency <= threshold.poor {
        ConnectionQuality::Poor
    } else {
        // If latency is higher than the 'poor' threshold, it's considered unusable
        // for many applications, effectively offline for quality purposes.
        ConnectionQuality::Offline
    }
}

/// Evaluates the final network quality by combining speed, stability, and loss.
pub fn evaluate_network_quality(
    is_connected: bool,
    stats: &LatencyStats,
    config: &NetworkConfiguration,
) -> ConnectionQuality {
    if !is_connected {
        return ConnectionQuality::Offline;
    }

    if stats.packet_loss_percent > config.resilience.critical_packet_loss_precent {
        return ConnectionQuality::Unstable;
    }

    let quality_based_on_speed = evaluate_quality(stats.latency_ms, &config.quality_threshold);

    if stats.stability_score < config.resilience.stability_thershold {
        return match quality_based_on_speed {
            ConnectionQuality::Excellent => ConnectionQuality::Great,
            ConnectionQuality::Great => ConnectionQuality::Good,
            ConnectionQuality::Good => ConnectionQuality::Moderate,
            ConnectionQuality::Moderate => ConnectionQuality::Poor,
            ConnectionQuality::Poor => ConnectionQuality::Unstable,
            other => other,
        };
    }

    if quality_based_on_speed == ConnectionQuality::Excellent && stats.stability_score < 85 {
        return ConnectionQuality::Great;
    }

    if quality_based_on_speed == ConnectionQuality::Great && stats.stability_score < 70 {
        return ConnectionQuality::Good;
    }

    quality_based_on_speed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::{
        ConnectionQuality, LatencyStats, NetworkConfiguration, QualityThresholds,
    };

    #[test]
    fn test_evaluate_quality_edge_cases() {
        let thresholds = QualityThresholds {
            excellent: 50,
            great: 100,
            good: 200,
            moderate: 400,
            poor: 1000,
        };

        // Zero case
        assert_eq!(
            evaluate_quality(0, &thresholds),
            ConnectionQuality::Excellent
        );

        // Exact boundary cases
        assert_eq!(
            evaluate_quality(50, &thresholds),
            ConnectionQuality::Excellent
        );
        assert_eq!(evaluate_quality(51, &thresholds), ConnectionQuality::Great);
        assert_eq!(evaluate_quality(100, &thresholds), ConnectionQuality::Great);
        assert_eq!(evaluate_quality(101, &thresholds), ConnectionQuality::Good);
        assert_eq!(evaluate_quality(200, &thresholds), ConnectionQuality::Good);
        assert_eq!(
            evaluate_quality(201, &thresholds),
            ConnectionQuality::Moderate
        );
        assert_eq!(
            evaluate_quality(400, &thresholds),
            ConnectionQuality::Moderate
        );
        assert_eq!(evaluate_quality(401, &thresholds), ConnectionQuality::Poor);
        assert_eq!(evaluate_quality(1000, &thresholds), ConnectionQuality::Poor);
        assert_eq!(
            evaluate_quality(1001, &thresholds),
            ConnectionQuality::Offline
        );
    }

    #[test]
    fn test_evaluate_network_quality_logic() {
        let mut config = NetworkConfiguration::default();
        config.resilience.stability_thershold = 50;
        config.resilience.critical_packet_loss_precent = 10.0;

        // Offline
        let stats = LatencyStats {
            latency_ms: 0,
            jitter_ms: 0,
            packet_loss_percent: 100.0,
            min_latency_ms: None,
            max_latency_ms: None,
            avg_latency_ms: None,
            stability_score: 0,
        };
        assert_eq!(
            evaluate_network_quality(false, &stats, &config),
            ConnectionQuality::Offline
        );

        // Critical loss
        let stats = LatencyStats {
            latency_ms: 100,
            jitter_ms: 0,
            packet_loss_percent: 20.0,
            min_latency_ms: Some(100),
            max_latency_ms: Some(100),
            avg_latency_ms: Some(100),
            stability_score: 80,
        };
        assert_eq!(
            evaluate_network_quality(true, &stats, &config),
            ConnectionQuality::Unstable
        );

        // Downgraded due to low stability score
        let stats = LatencyStats {
            latency_ms: 100,
            jitter_ms: 0,
            packet_loss_percent: 0.0,
            min_latency_ms: Some(100),
            max_latency_ms: Some(100),
            avg_latency_ms: Some(100),
            stability_score: 10,
        };
        assert_eq!(
            evaluate_network_quality(true, &stats, &config),
            ConnectionQuality::Good
        );
    }
}
