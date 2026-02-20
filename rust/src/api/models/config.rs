//! Configuration-related data structures.

use super::target::{NetworkTarget, TargetProtocol};
use crate::api::constants::LibConstants;

/// Defines the strategy for evaluating multiple targets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckStrategy {
    /// The first target to respond successfully determines the result.
    /// This is faster but less reliable.
    Race,
    /// A majority of targets must respond successfully for the check to be
    /// considered a success. This is slower but more robust.
    Consensus,
}

/// Represents the perceived quality of the network connection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionQuality {
    /// Excellent connection, very low latency. Suitable for all tasks.
    Excellent,
    /// Great connection, low latency. Suitable for most tasks.
    Great,
    /// Good, usable connection.
    Good,
    /// Moderate connection, noticeable latency. May affect real-time applications.
    Moderate,
    /// Poor connection, high latency. Basic browsing may be slow.
    Poor,
    /// Connection is active, but packet loss or high jitter makes it unreliable.
    Unstable,
    /// No connection detected or all essential targets failed.
    Offline,
}

/// Defines the latency thresholds (in milliseconds) used to determine [ConnectionQuality].
#[derive(Debug, Clone, Copy)]
pub struct QualityThresholds {
    /// Latency at or below this value is 'Excellent'.
    pub excellent: u64,
    /// Latency at or below this value is 'Great'.
    pub great: u64,
    /// Latency at or below this value is 'Good'.
    pub good: u64,
    /// Latency at or below this value is 'Moderate'.
    pub moderate: u64,
    /// Latency at or below this value is 'Poor'. Anything higher is 'Unstable'.
    pub poor: u64,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            excellent: LibConstants::DEFAULT_EXCELLENT_THRESHOLD,
            great: LibConstants::DEFAULT_GREAT_THRESHOLD,
            good: LibConstants::DEFAULT_GOOD_THRESHOLD,
            moderate: LibConstants::DEFAULT_MODERATE_THRESHOLD,
            poor: LibConstants::DEFAULT_POOR_THRESHOLD,
        }
    }
}

/// Configuration for security-related checks.
#[derive(Debug, Clone, Default)]
pub struct SecurityConfig {
    /// If true, the `guard` function will throw an exception if a VPN is detected.
    pub block_vpn: bool,
    /// If true, performs a check to detect potential DNS hijacking.
    /// This adds a small latency to each check.
    pub detect_dns_hijack: bool,
}

/// Configuration for resilience and performance tuning.
#[derive(Debug, Clone)]
pub struct ResilienceConfig {
    /// The strategy to use for checking multiple targets.
    pub strategy: CheckStrategy,
    /// The number of consecutive failures of essential targets before the
    /// circuit breaker opens. A value of 0 disables the circuit breaker.
    pub circuit_breaker_threshold: u8,
    /// The cooldown period in milliseconds after which the circuit breaker
    /// transitions from 'Open' to 'Half-Open'.
    pub circuit_breaker_cooldown_ms: u64,
    /// Number of samples to take for jitter and stability analysis.
    /// Must be greater than 1 to enable jitter calculation.
    pub num_jitter_samples: u8,
    /// The percentage of mean latency that the standard deviation must exceed
    /// to be considered high jitter, potentially downgrading quality.
    pub jitter_threshold_percent: f64,
    /// If the calculated stability score is less than this value, the quality considered 'Unstable'.
    pub stability_thershold: u8,
    /// The packet loss percentage above which the connection is marked as 'Unstable'.
    pub critical_packet_loss_precent: f32,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            strategy: CheckStrategy::Race,
            circuit_breaker_threshold: 0, // Disabled by default
            circuit_breaker_cooldown_ms: 60000, // 1 minute default
            num_jitter_samples: LibConstants::DEFAULT_JITTER_SAMPLES,
            jitter_threshold_percent: LibConstants::DEFAULT_JITTER_THRESHOLD_PERCENT,
            stability_thershold: LibConstants::DEFAULT_STABILITY_THRESHOLD,
            critical_packet_loss_precent: LibConstants::DEFAULT_CRITICAL_PACKET_LOSS_PRECENT,
        }
    }
}

/// The main configuration for the network reachability engine.
#[derive(Debug, Clone)]
pub struct NetworkConfiguration {
    /// A list of network endpoints to check.
    pub targets: Vec<NetworkTarget>,
    /// The time in milliseconds between automatic periodic checks.
    /// A value of 0 disables periodic checks.
    pub check_interval_ms: u64,
    /// The duration for which a network report is considered fresh (cached).
    pub cache_validity_ms: u64,
    /// Latency thresholds for determining connection quality.
    pub quality_threshold: QualityThresholds,
    /// Security-related settings.
    pub security: SecurityConfig,
    /// Resilience and performance tuning settings.
    pub resilience: ResilienceConfig,
}

impl Default for NetworkConfiguration {
    /// Creates a default configuration with checks against Cloudflare and Google DNS.
    fn default() -> Self {
        Self {
            targets: vec![
                NetworkTarget {
                    label: LibConstants::CLOUDFLARE_NAME.into(),
                    host: LibConstants::CLOUDFLARE_DNS.into(),
                    port: LibConstants::DEFAULT_PORT,
                    protocol: TargetProtocol::Tcp,
                    timeout_ms: LibConstants::DEFAULT_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
                NetworkTarget {
                    label: LibConstants::GOOGLE_NAME.into(),
                    host: LibConstants::GOOGLE_DNS.into(),
                    port: LibConstants::DEFAULT_PORT,
                    protocol: TargetProtocol::Tcp,
                    timeout_ms: LibConstants::DEFAULT_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
            ],
            check_interval_ms: LibConstants::DEFAULT_CHECK_INTERVAL_MS,
            cache_validity_ms: 2000, // 2 seconds default cache
            quality_threshold: QualityThresholds::default(),
            security: SecurityConfig::default(),
            resilience: ResilienceConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::constants::LibConstants;

    #[test]
    fn test_quality_thresholds_default() {
        let thresholds = QualityThresholds::default();
        assert_eq!(
            thresholds.excellent,
            LibConstants::DEFAULT_EXCELLENT_THRESHOLD
        );
        assert_eq!(thresholds.great, LibConstants::DEFAULT_GREAT_THRESHOLD);
        assert_eq!(thresholds.good, LibConstants::DEFAULT_GOOD_THRESHOLD);
        assert_eq!(
            thresholds.moderate,
            LibConstants::DEFAULT_MODERATE_THRESHOLD
        );
        assert_eq!(thresholds.poor, LibConstants::DEFAULT_POOR_THRESHOLD);
    }

    #[test]
    fn test_network_configuration_default() {
        let config = NetworkConfiguration::default();
        assert_eq!(config.targets.len(), 2);
        assert_eq!(config.targets[0].label, LibConstants::CLOUDFLARE_NAME);
        assert_eq!(config.targets[1].label, LibConstants::GOOGLE_NAME);
        assert_eq!(config.resilience.strategy, CheckStrategy::Race);
        assert!(!config.security.block_vpn);
    }
}
