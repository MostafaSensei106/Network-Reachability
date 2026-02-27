//! Configuration-related data structures.

use super::target::{NetworkTarget, TargetProtocol};
use crate::api::constants::LibConstants;

/// Defines the strategy used when evaluating multiple network targets.
///
/// This strategy determines how the engine decides if the network is "up"
/// when multiple targets are configured.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckStrategy {
    /// The first target to respond successfully determines the result.
    ///
    /// This is the fastest strategy as it doesn't wait for other targets.
    /// It's ideal for performance-sensitive applications where any connectivity
    /// to a trusted endpoint is sufficient.
    Race,
    /// A majority of targets must respond successfully for the check to be
    /// considered a success.
    ///
    /// This strategy is more robust against localized outages or transient
    /// failures of individual endpoints. It is slower than [Race] because
    /// it may wait for multiple responses.
    Consensus,
}

/// Represents the perceived quality of the network connection based on latency and stability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionQuality {
    /// Excellent connection with very low latency.
    ///
    /// Typically < 50ms. Suitable for high-performance real-time tasks like
    /// competitive gaming or high-frequency trading.
    Excellent,
    /// Great connection with low latency.
    ///
    /// Typically < 100ms. Suitable for VoIP, video conferencing, and smooth browsing.
    Great,
    /// Good, usable connection with acceptable latency.
    ///
    /// Typically < 150ms. Suitable for most standard web applications and streaming.
    Good,
    /// Moderate connection with noticeable latency.
    ///
    /// Typically < 250ms. Users might notice slight delays in interactive elements.
    Moderate,
    /// Poor connection with high latency.
    ///
    /// Typically < 500ms. Basic browsing will feel slow, and real-time apps will struggle.
    Poor,
    /// Connection is active, but high jitter or packet loss makes it unreliable.
    ///
    /// The network is "connected" but the experience will be degraded and unpredictable.
    Unstable,
    /// A captive portal (login page) was detected.
    ///
    /// The device is connected to an AP, but internet access is restricted until
    /// the user interacts with the portal (e.g., at an airport or hotel).
    CaptivePortal,
    /// No connection detected or all essential targets failed.
    ///
    /// The network is completely unreachable or unusable.
    Offline,
}

/// Defines the latency thresholds (in milliseconds) used to determine [ConnectionQuality].
///
/// These values allow customizing what "Good" or "Poor" means for your specific application.
#[derive(Debug, Clone, Copy)]
pub struct QualityThresholds {
    /// Latency at or below this value is considered [ConnectionQuality::Excellent].
    pub excellent: u64,
    /// Latency at or below this value is considered [ConnectionQuality::Great].
    pub great: u64,
    /// Latency at or below this value is considered [ConnectionQuality::Good].
    pub good: u64,
    /// Latency at or below this value is considered [ConnectionQuality::Moderate].
    pub moderate: u64,
    /// Latency at or below this value is considered [ConnectionQuality::Poor].
    /// Anything higher is categorized as [ConnectionQuality::Unstable] or [ConnectionQuality::Poor].
    pub poor: u64,
}

impl QualityThresholds {
    /// Creates a new [QualityThresholds] instance.
    pub fn new(excellent: u64, great: u64, good: u64, moderate: u64, poor: u64) -> Self {
        Self {
            excellent,
            great,
            good,
            moderate,
            poor,
        }
    }
}

/// Provides sensible default latency thresholds for most mobile and web applications.
///
/// Defaults:
/// - Excellent: 50ms
/// - Great: 100ms
/// - Good: 150ms
/// - Moderate: 250ms
/// - Poor: 500ms
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

/// Configuration for security-related network checks.
#[derive(Debug, Clone, Default)]
pub struct SecurityConfig {
    /// If true, the reachability engine will flag or block connections if a VPN is detected.
    ///
    /// This is useful for region-locked content or compliance requirements.
    pub block_vpn: bool,
    /// If true, performs a check to detect potential DNS hijacking.
    ///
    /// This involves comparing results from the system DNS with a trusted resolver.
    /// Note: This adds a small amount of latency to each check.
    pub detect_dns_hijack: bool,
}

/// Configuration for resilience, stability analysis, and performance tuning.
#[derive(Debug, Clone)]
pub struct ResilienceConfig {
    /// The evaluation strategy to use when checking multiple targets.
    pub strategy: CheckStrategy,
    /// The number of consecutive failures of essential targets before the
    /// circuit breaker opens.
    ///
    /// A value of 0 disables the circuit breaker mechanism.
    pub circuit_breaker_threshold: u8,
    /// The cooldown period in milliseconds.
    ///
    /// After this time, the circuit breaker transitions from 'Open' to 'Half-Open',
    /// allowing a trial check to see if the network has recovered.
    pub circuit_breaker_cooldown_ms: u64,
    /// Number of samples to take for jitter and stability analysis.
    ///
    /// Higher values provide more accurate stability metrics but increase check duration.
    /// Must be > 1 to enable jitter calculation.
    pub num_jitter_samples: u8,
    /// The percentage of mean latency that the standard deviation must exceed
    /// to be flagged as high jitter.
    ///
    /// High jitter can downgrade the perceived [ConnectionQuality] to [ConnectionQuality::Unstable].
    pub jitter_threshold_percent: f64,
    /// The minimum stability score (0-100) required to avoid the 'Unstable' quality tag.
    pub stability_thershold: u8,
    /// The packet loss percentage above which the connection is marked as [ConnectionQuality::Unstable].
    pub critical_packet_loss_precent: f32,
}

impl ResilienceConfig {
    /// Creates a new [ResilienceConfig] instance.
    pub fn new(
        strategy: CheckStrategy,
        circuit_breaker_threshold: u8,
        circuit_breaker_cooldown_ms: u64,
        num_jitter_samples: u8,
        jitter_threshold_percent: f64,
        stability_thershold: u8,
        critical_packet_loss_precent: f32,
    ) -> Self {
        Self {
            strategy,
            circuit_breaker_threshold,
            circuit_breaker_cooldown_ms,
            num_jitter_samples,
            jitter_threshold_percent,
            stability_thershold,
            critical_packet_loss_precent,
        }
    }
}

/// Provides a standard resilience configuration balanced for stability and speed.
///
/// Defaults:
/// - Strategy: [CheckStrategy::Race]
/// - Circuit Breaker: Disabled (threshold 0)
/// - Cooldown: 1 minute
/// - Jitter Samples: 5
/// - Stability Threshold: 80
impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            strategy: CheckStrategy::Race,
            circuit_breaker_threshold: 0,       // Disabled by default
            circuit_breaker_cooldown_ms: 60000, // 1 minute default
            num_jitter_samples: LibConstants::DEFAULT_JITTER_SAMPLES,
            jitter_threshold_percent: LibConstants::DEFAULT_JITTER_THRESHOLD_PERCENT,
            stability_thershold: LibConstants::DEFAULT_STABILITY_THRESHOLD,
            critical_packet_loss_precent: LibConstants::DEFAULT_CRITICAL_PACKET_LOSS_PRECENT,
        }
    }
}

/// The main configuration object for the network reachability engine.
///
/// This struct aggregates all settings including targets, intervals,
/// quality thresholds, and security/resilience preferences.
#[derive(Debug, Clone)]
pub struct NetworkConfiguration {
    /// A list of network endpoints ([NetworkTarget]) to be monitored.
    pub targets: Vec<NetworkTarget>,
    /// The interval in milliseconds between automatic periodic checks.
    ///
    /// Set to 0 to disable periodic background checks and rely on manual triggers.
    pub check_interval_ms: u64,
    /// The duration (in milliseconds) for which a network report is cached.
    ///
    /// Rapid consecutive calls will return the cached report to save battery and bandwidth.
    pub cache_validity_ms: u64,
    /// Custom latency thresholds for determining connection quality.
    pub quality_threshold: QualityThresholds,
    /// Security-specific settings such as VPN and DNS hijacking detection.
    pub security: SecurityConfig,
    /// Resilience settings including circuit breakers and jitter analysis.
    pub resilience: ResilienceConfig,
}

impl NetworkConfiguration {
    /// Creates a new [NetworkConfiguration] instance.
    pub fn new(
        targets: Vec<NetworkTarget>,
        check_interval_ms: u64,
        cache_validity_ms: u64,
        quality_threshold: QualityThresholds,
        security: SecurityConfig,
        resilience: ResilienceConfig,
    ) -> Self {
        Self {
            targets,
            check_interval_ms,
            cache_validity_ms,
            quality_threshold,
            security,
            resilience,
        }
    }
}

/// Creates a production-ready default configuration.
///
/// The default setup includes:
/// - Monitoring multiple high-availability targets (Cloudflare and Google).
/// - Automatic periodic checks every 5 seconds.
/// - 2-second result caching to prevent redundant probes.
impl Default for NetworkConfiguration {
    fn default() -> Self {
        Self {
            targets: vec![
                NetworkTarget {
                    label: LibConstants::CLOUDFLARE_NAME_HTTP.into(),
                    host: LibConstants::CLOUDFLARE_DNS.into(),
                    port: LibConstants::DEFAULT_HTTP_PORT,
                    protocol: TargetProtocol::Http,
                    timeout_ms: LibConstants::DEFAULT_HTTP_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
                NetworkTarget {
                    label: LibConstants::CLOUDFLARE_NAME_HTTPS.into(),
                    host: LibConstants::CLOUDFLARE_DNS.into(),
                    port: LibConstants::DEFAULT_HTTP_PORT,
                    protocol: TargetProtocol::Https,
                    timeout_ms: LibConstants::DEFAULT_HTTP_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
                NetworkTarget {
                    label: LibConstants::CLOUDFLARE_NAME.into(),
                    host: LibConstants::GOOGLE_DNS.into(),
                    port: LibConstants::DEFAULT_PORT,
                    protocol: TargetProtocol::Tcp,
                    timeout_ms: LibConstants::DEFAULT_HTTP_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
                NetworkTarget {
                    label: LibConstants::CLOUDFLARE_NAME.into(),
                    host: LibConstants::GOOGLE_DNS.into(),
                    port: LibConstants::DEFAULT_PORT,
                    protocol: TargetProtocol::Icmp,
                    timeout_ms: LibConstants::DEFAULT_HTTP_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
            ],
            check_interval_ms: LibConstants::DEFAULT_CHECK_INTERVAL_MS,
            cache_validity_ms: LibConstants::DEFAULT_CACHE_VALIDITY_MS, // 2 seconds default cache
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
        assert_eq!(config.targets.len(), 4);
        assert_eq!(config.targets[0].label, LibConstants::CLOUDFLARE_NAME_HTTP);
        assert_eq!(config.targets[1].label, LibConstants::CLOUDFLARE_NAME_HTTPS);
        assert_eq!(config.resilience.strategy, CheckStrategy::Race);
        assert!(!config.security.block_vpn);
    }
}
