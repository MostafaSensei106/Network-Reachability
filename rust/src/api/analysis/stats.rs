//! # Network Statistics & Scoring
//!
//! This module handles the heavy lifting of statistical analysis. It transforms
//! a collection of raw latency samples into meaningful metrics like Jitter,
//! P95 Latency, and a consolidated Stability Score.

use crate::api::models::{LatencyStats, QualityThresholds};

/// Calculates basic statistical metrics for a set of latency samples.
///
/// Useful for quick analysis of a probe cycle.
///
/// # Returns
///
/// A tuple: `(Min, Max, Mean, StandardDeviation)`.
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

    // Sample standard deviation (Bessel's correction)
    let std_dev = (variance_sum / (count as f64 - 1.0)).sqrt();

    (
        Some(min_latency),
        Some(max_latency),
        Some(mean_latency),
        Some(std_dev),
    )
}

/// Computes a comprehensive stability report and health score.
///
/// The scoring system is specifically tuned for modern mobile (4G/5G) and
/// WiFi networks. It uses a weighted composite model:
///
/// ### Scoring Weights:
/// * **P95 Latency (35%):** Penalizes "tail latency" (occasional slow packets).
/// * **Packet Loss (30%):** Heavily penalizes unreliability.
/// * **Mean Latency (20%):** Baseline speed assessment.
/// * **IQR Jitter (15%):** Measures arrival consistency using Interquartile Range.
///
/// # Arguments
/// * `latencies`: Successful probe results.
/// * `total_expected_samples`: Used to calculate packet loss.
/// * `thresholds`: User-defined latency boundaries.
pub fn compute_latency_stats(
    latencies: &[u64],
    total_expected_samples: u8,
    thresholds: &QualityThresholds,
) -> LatencyStats {
    let successful_samples = latencies.len() as f32;
    let packet_loss_percent = if total_expected_samples > 0 {
        let loss = 100.0 * (1.0 - (successful_samples / total_expected_samples as f32));
        loss.max(0.0)
    } else {
        0.0
    };

    if latencies.is_empty() {
        return LatencyStats {
            latency_ms: 0,
            jitter_ms: 0,
            packet_loss_percent,
            min_latency_ms: None,
            max_latency_ms: None,
            avg_latency_ms: None,
            stability_score: 0,
        };
    }

    let mut sorted = latencies.to_vec();
    sorted.sort_unstable();

    let n = sorted.len();
    let mean_f64: f64 = sorted.iter().sum::<u64>() as f64 / n as f64;
    let mean_ms = mean_f64.round() as u64;

    // Helper for calculating N-th percentile
    let percentile = |p: f64| -> f64 {
        if n == 1 {
            return sorted[0] as f64;
        }
        let rank = p / 100.0 * (n - 1) as f64;
        let lo = rank.floor() as usize;
        let hi = (lo + 1).min(n - 1);
        let frac = rank - lo as f64;
        sorted[lo] as f64 + frac * (sorted[hi] as f64 - sorted[lo] as f64)
    };

    let p25 = percentile(25.0);
    let p75 = percentile(75.0);
    let p95 = percentile(95.0);
    let min_lat = sorted.first().copied();
    let max_lat = sorted.last().copied();

    // IQR is more robust against outliers than standard deviation for small samples
    let iqr_jitter = p75 - p25;

    let variance: f64 = if n > 1 {
        sorted
            .iter()
            .map(|&x| {
                let d = x as f64 - mean_f64;
                d * d
            })
            .sum::<f64>()
            / (n - 1) as f64
    } else {
        0.0
    };
    let std_dev = variance.sqrt();

    // Stepped scorer: maps a latency value to a 0-100 score based on user thresholds
    let score_latency = |ms: f64| -> f64 {
        let t = &thresholds;
        let (ex, gr, go, mo, po) = (
            t.excellent as f64,
            t.great as f64,
            t.good as f64,
            t.moderate as f64,
            t.poor as f64,
        );

        let unusable = po * 2.0;

        if ms <= ex {
            lerp(100.0, 88.0, ms, 0.0, ex)
        } else if ms <= gr {
            lerp(88.0, 72.0, ms, ex, gr)
        } else if ms <= go {
            lerp(72.0, 52.0, ms, gr, go)
        } else if ms <= mo {
            lerp(52.0, 28.0, ms, go, mo)
        } else if ms <= po {
            lerp(28.0, 8.0, ms, mo, po)
        } else {
            lerp(8.0, 0.0, ms.min(unusable), po, unusable)
        }
    };

    let p95_score = score_latency(p95);

    // Loss Scorer: sharp drops after 1% loss, zero score after 15%
    let loss_f64 = packet_loss_percent as f64;
    let loss_score: f64 = if loss_f64 <= 0.0 {
        100.0
    } else if loss_f64 <= 1.0 {
        lerp(100.0, 80.0, loss_f64, 0.0, 1.0)
    } else if loss_f64 <= 5.0 {
        lerp(80.0, 50.0, loss_f64, 1.0, 5.0)
    } else if loss_f64 <= 15.0 {
        lerp(50.0, 15.0, loss_f64, 5.0, 15.0)
    } else {
        0.0
    };

    let mean_score = score_latency(mean_f64);

    // Jitter Scorer: penalizes relative variance
    let jitter_score: f64 = if mean_f64 > f64::EPSILON {
        let relative_iqr = (iqr_jitter / mean_f64) * 100.0;
        if relative_iqr <= 10.0 {
            lerp(100.0, 90.0, relative_iqr, 0.0, 10.0)
        } else if relative_iqr <= 30.0 {
            lerp(90.0, 65.0, relative_iqr, 10.0, 30.0)
        } else if relative_iqr <= 60.0 {
            lerp(65.0, 30.0, relative_iqr, 30.0, 60.0)
        } else if relative_iqr <= 100.0 {
            lerp(30.0, 5.0, relative_iqr, 60.0, 100.0)
        } else {
            0.0
        }
    } else {
        100.0
    };

    // Calculate Final Weighted Score
    let mut weighted_score =
        p95_score * 0.35 + loss_score * 0.30 + mean_score * 0.20 + jitter_score * 0.15;

    // Severe penalty for > 50% loss (network is effectively dying)
    if packet_loss_percent > 50.0 {
        let survival = (100.0 - packet_loss_percent as f64) / 50.0;
        weighted_score *= survival;
    }

    LatencyStats {
        latency_ms: mean_ms,
        jitter_ms: std_dev as u64,
        packet_loss_percent,
        min_latency_ms: min_lat,
        max_latency_ms: max_lat,
        avg_latency_ms: Some(mean_ms),
        stability_score: weighted_score.clamp(0.0, 100.0) as u8,
    }
}

/// Helper: Performs linear interpolation between two points.
#[inline]
fn lerp(out_min: f64, out_max: f64, value: f64, in_min: f64, in_max: f64) -> f64 {
    if (in_max - in_min).abs() < f64::EPSILON {
        return out_min;
    }
    let t = (value - in_min) / (in_max - in_min);
    (out_min + t * (out_max - out_min)).clamp(out_max.min(out_min), out_max.max(out_min))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_jitter_stats_edge_cases() {
        // Zero latencies
        assert_eq!(calculate_jitter_stats(&[]), (None, None, None, None));

        // Increasing jitter
        let latencies = vec![100, 200, 300, 400, 500];
        let (_, _, mean, std_dev) = calculate_jitter_stats(&latencies);
        assert_eq!(mean, Some(300));
        assert!(std_dev.unwrap() > 100.0);

        // All identical (zero jitter)
        let latencies = vec![100, 100, 100];
        let (_, _, mean, std_dev) = calculate_jitter_stats(&latencies);
        assert_eq!(mean, Some(100));
        assert_eq!(std_dev, Some(0.0));
    }
}
