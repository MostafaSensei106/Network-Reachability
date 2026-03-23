//! Configuration-related data structures for the network reachability engine.
//!
//! This module contains the core configuration types that control how the engine
//! performs network checks, evaluates quality, and handles failures.

use super::target::{NetworkTarget, TargetProtocol};
use crate::api::constants::LibConstants;

/// Defines the strategy used when evaluating multiple network targets during a check cycle.
///
/// When the engine is configured with multiple targets, the `CheckStrategy` determines
/// the logic for deciding the overall "connected" status of the network.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckStrategy {
    /// The first target to respond successfully determines the result.
    ///
    /// # Behavior
    /// In this mode, the engine initiates checks for all configured targets concurrently.
    /// As soon as a single target returns a successful response, the network is marked
    /// as "connected" and the engine returns the result immediately.
    ///
    /// # Use Case
    /// This is the fastest strategy and is ideal for performance-sensitive applications
    /// where knowing *any* path to the internet is open is sufficient. It minimizes
    /// latency for the check itself.
    Race,

    /// A majority of targets must respond successfully for the check to be considered a success.
    ///
    /// # Behavior
    /// The engine waits for a quorum of targets to respond. For example, if 3 targets
    /// are configured, at least 2 must succeed for the overall status to be "connected".
    /// If the majority fails, the network is considered "offline" or "unstable".
    ///
    /// # Use Case
    /// This strategy provides high robustness against transient failures of specific
    /// servers (e.g., a specific DNS provider being down). It's best for critical
    /// applications that require high confidence in the network's reliability.
    Consensus,
}

/// Represents the perceived quality of the network connection based on latency, jitter, and stability.
///
/// The engine maps raw metrics (like RTT in milliseconds) to these categories using
/// the thresholds defined in [`QualityThresholds`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionQuality {
    /// Excellent connection with very low latency and high stability.
    ///
    /// Typically < 50ms.
    /// **User Experience:** Instantaneous page loads, seamless 4K streaming, and
    /// zero-lag competitive gaming.
    Excellent,

    /// Great connection with low latency.
    ///
    /// Typically < 100ms.
    /// **User Experience:** Fast browsing, high-quality video calls (VoIP/Zoom)
    /// without noticeable delay.
    Great,

    /// Good, usable connection with acceptable latency.
    ///
    /// Typically < 150ms.
    /// **User Experience:** Reliable for most tasks, though large file uploads
    /// or fast-paced games might show slight degradation.
    Good,

    /// Moderate connection with noticeable latency.
    ///
    /// Typically < 250ms.
    /// **User Experience:** Noticeable delays when opening new pages. Video streaming
    /// might occasionally buffer at the start.
    Moderate,

    /// Poor connection with high latency.
    ///
    /// Typically < 500ms.
    /// **User Experience:** Frustratingly slow. Interactive applications feel
    /// "heavy" and unresponsive.
    Poor,

    /// Connection is active, but high jitter or packet loss makes it unreliable.
    ///
    /// This state occurs when latency is technically okay, but the "consistency"
    /// is missing (e.g., standard deviation of samples is too high).
    /// **User Experience:** "Stuttering" in calls, frequent disconnects,
    /// and unpredictable performance.
    Unstable,

    /// A captive portal (login page) was detected.
    ///
    /// The device is connected to a WiFi access point, but internet access is
    /// intercepted by a gateway (common in hotels/airports).
    /// **User Experience:** No internet access until the user signs in or
    /// accepts terms.
    CaptivePortal,

    /// No connection detected or all essential targets failed.
    ///
    /// **User Experience:** The app should enter its "Offline Mode".
    Offline,
}

/// Defines the latency thresholds (in milliseconds) used to categorize [`ConnectionQuality`].
///
/// These values act as the "buckets" that convert raw Round-Trip Time (RTT) values
/// into user-friendly quality ratings.
#[derive(Debug, Clone, Copy)]
pub struct QualityThresholds {
    /// Maximum latency (ms) to be considered [`ConnectionQuality::Excellent`].
    /// *Default: 50ms*
    pub excellent: u64,
    /// Maximum latency (ms) to be considered [`ConnectionQuality::Great`].
    /// *Default: 100ms*
    pub great: u64,
    /// Maximum latency (ms) to be considered [`ConnectionQuality::Good`].
    /// *Default: 150ms*
    pub good: u64,
    /// Maximum latency (ms) to be considered [`ConnectionQuality::Moderate`].
    /// *Default: 250ms*
    pub moderate: u64,
    /// Maximum latency (ms) to be considered [`ConnectionQuality::Poor`].
    /// Anything above this is typically marked as [`ConnectionQuality::Poor`] or
    /// [`ConnectionQuality::Unstable`].
    /// *Default: 500ms*
    pub poor: u64,
}

impl QualityThresholds {
    /// Creates a new custom [`QualityThresholds`] instance.
    ///
    /// # Arguments
    /// * `excellent` - Threshold for 'Excellent' (ms).
    /// * `great` - Threshold for 'Great' (ms).
    /// * `good` - Threshold for 'Good' (ms).
    /// * `moderate` - Threshold for 'Moderate' (ms).
    /// * `poor` - Threshold for 'Poor' (ms).
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

/// Provides industry-standard default latency thresholds for most applications.
///
/// These defaults are tuned for general-purpose mobile and web apps:
/// - **Excellent:** 50ms (Fiber/High-speed Cable)
/// - **Great:** 100ms (Average Broadband)
/// - **Good:** 150ms (Stable 4G/LTE)
/// - **Moderate:** 250ms (3G/Slower Satellite)
/// - **Poor:** 500ms (Highly congested or edge networks)
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

/// Configuration for security-related network checks and policy enforcement.
///
/// These settings allow the engine to detect environmental factors that might
/// be undesirable or indicate a compromised connection.
#[derive(Debug, Clone, Default)]
pub struct SecurityConfig {
    /// If enabled, the engine will flag connections that originate from a VPN interface.
    ///
    /// Useful for applications that enforce geo-fencing or need to prevent
    /// identity masking.
    pub block_vpn: bool,

    /// If enabled, performs deep DNS validation to detect hijacking.
    ///
    /// The engine will compare the results of the local system resolver with
    /// a trusted upstream resolver (like Cloudflare or Google). If they differ
    /// significantly for static domains, it flags a potential spoofing attempt.
    pub detect_dns_hijack: bool,
}

/// Configuration for network resilience, failure handling, and statistical analysis.
///
/// This struct controls the "brain" of the engine: how it handles noise,
/// how it reacts to failure, and how it calculates jitter.
#[derive(Debug, Clone)]
pub struct ResilienceConfig {
    /// The evaluation strategy (Race vs Consensus) for multi-target checks.
    pub strategy: CheckStrategy,

    /// Number of consecutive failures before the "Circuit Breaker" opens.
    ///
    /// When the circuit breaker is 'Open', the engine stops sending probes to
    /// save resources and battery, assuming the network is definitely down.
    /// *Set to 0 to disable.*
    pub circuit_breaker_threshold: u8,

    /// Duration (ms) the engine waits before attempting a "Half-Open" probe.
    ///
    /// Once the circuit breaker is open, it waits for this cooldown before
    /// trying one more check to see if connectivity has returned.
    pub circuit_breaker_cooldown_ms: u64,

    /// Number of packets/samples to send per target for statistical analysis.
    ///
    /// Higher values (e.g., 10+) provide extremely accurate jitter and packet loss
    /// metrics but increase the battery/data usage and duration of each check.
    /// *Minimum 2 required for jitter calculation.*
    pub num_jitter_samples: u8,

    /// Percentage threshold for jitter classification.
    ///
    /// If the standard deviation of latency samples divided by the mean exceeds
    /// this percentage, the connection is marked as 'Unstable'.
    pub jitter_threshold_percent: f64,

    /// Minimum stability score (0-100) required for a 'Stable' rating.
    ///
    /// A score calculated from packet loss and jitter consistency.
    pub stability_thershold: u8,

    /// Critical packet loss percentage (0.0 - 100.0).
    ///
    /// If packet loss exceeds this value, the connection is immediately
    /// downgraded to 'Unstable' or 'Offline'.
    pub critical_packet_loss_precent: f32,
}

impl ResilienceConfig {
    /// Creates a new custom [`ResilienceConfig`].
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

/// Balanced default resilience configuration.
///
/// - Strategy: [`CheckStrategy::Race`] (optimized for speed)
/// - Jitter Samples: 5 (good balance of accuracy and speed)
/// - Circuit Breaker: Disabled by default.
impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            strategy: CheckStrategy::Race,
            circuit_breaker_threshold: 0,
            circuit_breaker_cooldown_ms: 60000, // 1 minute
            num_jitter_samples: LibConstants::DEFAULT_JITTER_SAMPLES,
            jitter_threshold_percent: LibConstants::DEFAULT_JITTER_THRESHOLD_PERCENT,
            stability_thershold: LibConstants::DEFAULT_STABILITY_THRESHOLD,
            critical_packet_loss_precent: LibConstants::DEFAULT_CRITICAL_PACKET_LOSS_PRECENT,
        }
    }
}

/// The master configuration for the Network Reachability Engine.
///
/// This structure is the entry point for customizing how the engine behaves.
/// It should be initialized once and passed to the engine during startup.
#[derive(Debug, Clone)]
pub struct NetworkConfiguration {
    /// The list of endpoints ([`NetworkTarget` models](super::target::NetworkTarget))
    /// to probe.
    pub targets: Vec<NetworkTarget>,

    /// Frequency (ms) of background checks.
    ///
    /// If set to 5000, the engine will automatically run a check every 5 seconds
    /// and stream results. Set to 0 to disable periodic checks.
    pub check_interval_ms: u64,

    /// Cache duration (ms) for results.
    ///
    /// If a manual check is requested within this window of a previous check,
    /// the engine will return the cached result instead of performing new
    /// network I/O. This saves significant battery and data.
    pub cache_validity_ms: u64,

    /// Thresholds for quality categorization.
    pub quality_threshold: QualityThresholds,

    /// Security policy settings.
    pub security: SecurityConfig,

    /// Performance and resilience settings.
    pub resilience: ResilienceConfig,
}

impl NetworkConfiguration {
    /// Constructs a full [`NetworkConfiguration`].
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

/// Standard production-ready configuration.
///
/// Includes:
/// - **Targets:** Cloudflare (HTTP/HTTPS/TCP/ICMP) and Google (TCP/ICMP).
/// - **Interval:** 5 seconds.
/// - **Cache:** 2 seconds.
/// - **Defaults:** Balanced quality and resilience settings.
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
                    label: "Cloudflare ICMP".into(),
                    host: LibConstants::CLOUDFLARE_DNS.into(),
                    port: 0,
                    protocol: TargetProtocol::Icmp,
                    timeout_ms: LibConstants::DEFAULT_HTTP_TIMEOUT_MS,
                    priority: 1,
                    is_essential: false,
                },
            ],
            check_interval_ms: LibConstants::DEFAULT_CHECK_INTERVAL_MS,
            cache_validity_ms: LibConstants::DEFAULT_CACHE_VALIDITY_MS,
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
