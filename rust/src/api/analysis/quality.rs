use crate::api::models::{ConnectionQuality, QualityThresholds};

/// Evaluates connection quality based on latency against a set of thresholds.
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
        ConnectionQuality::Offline
    }
}

/// Calculates statistical metrics for a series of latency samples.
///
/// Returns a tuple containing: (min, max, mean, standard_deviation).
pub fn calculate_jitter_stats(
    latencies: &[u64],
) -> (Option<u64>, Option<u64>, Option<u64>, Option<f64>) {
    if latencies.is_empty() {
        return (None, None, None, None);
    }

    let min_latency = *latencies.iter().min().unwrap();
    let max_latency = *latencies.iter().max().unwrap();

    let sum: u64 = latencies.iter().sum();
    let count = latencies.len() as u64;
    let mean_latency = sum / count;

    if count < 2 {
        // Standard deviation requires at least 2 samples
        return (
            Some(min_latency),
            Some(max_latency),
            Some(mean_latency),
            None,
        );
    }

    let variance_sum: f64 = latencies
        .iter()
        .map(|&x| {
            let diff = (x as f64) - (mean_latency as f64);
            diff * diff
        })
        .sum();
    let std_dev = (variance_sum / (count as f64 - 1.0)).sqrt();

    (
        Some(min_latency),
        Some(max_latency),
        Some(mean_latency),
        Some(std_dev),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::QualityThresholds;

    #[test]
    fn test_evaluate_quality_logic() {
        let thresholds = QualityThresholds {
            excellent: 50,
            great: 100,
            good: 200,
            moderate: 400,
            poor: 1000,
        };

        assert_eq!(
            evaluate_quality(49, &thresholds),
            ConnectionQuality::Excellent
        );
        assert_eq!(
            evaluate_quality(50, &thresholds),
            ConnectionQuality::Excellent
        );
        assert_eq!(evaluate_quality(100, &thresholds), ConnectionQuality::Great);
        assert_eq!(
            evaluate_quality(250, &thresholds),
            ConnectionQuality::Moderate
        );
        assert_eq!(
            evaluate_quality(1001, &thresholds),
            ConnectionQuality::Offline
        );
    }

    #[test]
    fn test_calculate_jitter_stats_logic() {
        // Empty case
        let (min, max, mean, std_dev) = calculate_jitter_stats(&[]);
        assert_eq!(min, None);
        assert_eq!(max, None);
        assert_eq!(mean, None);
        assert_eq!(std_dev, None);

        // Single item case
        let (min, max, mean, std_dev) = calculate_jitter_stats(&[100]);
        assert_eq!(min, Some(100));
        assert_eq!(max, Some(100));
        assert_eq!(mean, Some(100));
        assert_eq!(std_dev, None);

        // Normal case
        let latencies = vec![100, 110, 90, 105, 95];
        let (min, max, mean, std_dev) = calculate_jitter_stats(&latencies);

        assert_eq!(min, Some(90));
        assert_eq!(max, Some(110));
        assert_eq!(mean, Some(100));
        assert!(std_dev.is_some());
        assert!((std_dev.unwrap() - 7.905).abs() < 0.01);

        // Zero jitter case
        let latencies = vec![50, 50, 50, 50];
        let (_, _, _, std_dev) = calculate_jitter_stats(&latencies);
        assert!(std_dev.is_some());
        assert!((std_dev.unwrap() - 0.0).abs() < 0.01);
    }
}
