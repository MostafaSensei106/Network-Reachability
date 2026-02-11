use super::target::{NetworkTarget, TargetProtocol};
use crate::api::constants::AppConstants;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckStrategy {
    /// The first target to respond successfully determines the result.
    Race,
    /// A majority of targets must respond successfully.
    Consensus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionQuality {
    Excellent,
    Great,
    Good,
    Moderate,
    Poor,
    /// Connection is active but latency is highly variable.
    Unstable,
    /// No connection.
    Dead,
}

#[derive(Debug, Clone, Copy)]
pub struct QualityThresholds {
    pub excellent: u64,
    pub great: u64,
    pub good: u64,
    pub moderate: u64,
    pub poor: u64,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            excellent: AppConstants::DEFAULT_EXCELLENT_THRESHOLD,
            great: AppConstants::DEFAULT_GREAT_THRESHOLD,
            good: AppConstants::DEFAULT_GOOD_THRESHOLD,
            moderate: AppConstants::DEFAULT_MODERATE_THRESHOLD,
            poor: AppConstants::DEFAULT_POOR_THRESHOLD,
        }
    }
}

/// Configuration for security-related checks.
#[derive(Debug, Clone, Default)]
pub struct SecurityConfig {
    /// If true, the check will fail if a VPN is detected.
    pub block_vpn: bool,
    /// If true, performs a check to detect potential DNS hijacking.
    pub detect_dns_hijack: bool,
    /// A list of allowed interface prefixes. If not empty, the check will fail if the active interface is not on this list.
    pub allowed_interfaces: Vec<String>,
}

/// Configuration for resilience and performance features.
#[derive(Debug, Clone)]
pub struct ResilienceConfig {
    /// The strategy to use for checking multiple targets.
    pub strategy: CheckStrategy,
    /// The number of consecutive failures before the circuit breaker opens. 0 to disable.
    pub circuit_breaker_threshold: u8,
    /// Number of samples for jitter analysis. Must be > 1 to enable jitter check.
    pub num_jitter_samples: u8,
    /// The percentage of mean latency that the standard deviation must exceed to be marked as 'Unstable'.
    pub jitter_threshold_percent: f64,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            strategy: CheckStrategy::Race,
            circuit_breaker_threshold: 0, // Disabled by default
            num_jitter_samples: AppConstants::DEFAULT_JITTER_SAMPLES,
            jitter_threshold_percent: AppConstants::DEFAULT_JITTER_THRESHOLD_PERCENT,
        }
    }
}

/// The main configuration for the network reachability engine.
#[derive(Debug, Clone)]
pub struct NetworkConfiguration {
    /// The list of targets to check.
    pub targets: Vec<NetworkTarget>,
    /// The time between automatic checks. 0 to disable.
    pub check_interval_ms: u64,
    /// Latency thresholds for determining connection quality.
    pub quality_threshold: QualityThresholds,
    /// Security-related settings.
    pub security: SecurityConfig,
    /// Resilience and performance settings.
    pub resilience: ResilienceConfig,
}

impl Default for NetworkConfiguration {
    fn default() -> Self {
        Self {
            targets: vec![
                NetworkTarget {
                    label: AppConstants::CLOUDFLARE_NAME.into(),
                    host: AppConstants::CLOUDFLARE_DNS.into(),
                    port: AppConstants::DEFAULT_PORT,
                    protocol: TargetProtocol::Tcp,
                    timeout_ms: AppConstants::DEFAULT_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
                NetworkTarget {
                    label: AppConstants::GOOGLE_NAME.into(),
                    host: AppConstants::GOOGLE_DNS.into(),
                    port: AppConstants::DEFAULT_PORT,
                    protocol: TargetProtocol::Tcp,
                    timeout_ms: AppConstants::DEFAULT_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
            ],
            check_interval_ms: AppConstants::DEFAULT_CHECK_INTERVAL_MS,
            quality_threshold: QualityThresholds::default(),
            security: SecurityConfig::default(),
            resilience: ResilienceConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::constants::AppConstants;

    #[test]
    fn test_quality_thresholds_default() {
        let thresholds = QualityThresholds::default();
        assert_eq!(
            thresholds.excellent,
            AppConstants::DEFAULT_EXCELLENT_THRESHOLD
        );
        assert_eq!(thresholds.great, AppConstants::DEFAULT_GREAT_THRESHOLD);
        assert_eq!(thresholds.good, AppConstants::DEFAULT_GOOD_THRESHOLD);
        assert_eq!(
            thresholds.moderate,
            AppConstants::DEFAULT_MODERATE_THRESHOLD
        );
        assert_eq!(thresholds.poor, AppConstants::DEFAULT_POOR_THRESHOLD);
    }

    #[test]
    fn test_network_configuration_default() {
        let config = NetworkConfiguration::default();
        assert_eq!(config.targets.len(), 2);
        assert_eq!(config.targets[0].label, AppConstants::CLOUDFLARE_NAME);
        assert_eq!(config.targets[1].label, AppConstants::GOOGLE_NAME);
        assert_eq!(config.resilience.strategy, CheckStrategy::Race);
        assert!(!config.security.block_vpn);
    }
}
