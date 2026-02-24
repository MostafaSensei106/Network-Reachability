//! Data structures for the final, consolidated network check reports.

use crate::api::models::SecurityFlagsResult;

use super::config::ConnectionQuality;
use super::net_info::ConnectionType;

/// Detailed outcome of a connectivity check against a specific [NetworkTarget].
#[derive(Debug, Clone)]
pub struct TargetReport {
    /// The unique label identifying the target.
    pub label: String,
    /// Indicates whether the target was successfully reached using the specified protocol.
    pub success: bool,
    /// The measured latency in milliseconds.
    ///
    /// This value is 0 or undefined if [success] is false.
    pub latency_ms: u64,
    /// A descriptive error message if the check failed.
    pub error: Option<String>,
    /// Whether this target was marked as essential for the overall network status.
    pub is_essential: bool,
}

/// A suite of statistical metrics derived from multiple latency samples.
///
/// These metrics provide a deeper look into the stability and consistency
/// of the connection beyond a simple "is it up?" check.
#[derive(Debug, Clone)]
pub struct LatencyStats {
    /// The representative latency value for this report (usually the mean or median of samples).
    pub latency_ms: u64,
    /// The standard deviation of latency samples, measuring "jitter".
    ///
    /// High jitter (e.g., > 30ms) can significantly degrade the experience in real-time apps.
    pub jitter_ms: u64,
    /// The percentage of samples that failed to return a response.
    ///
    /// Any value above 0.0% indicates potential network congestion or hardware issues.
    pub packet_loss_percent: f32,
    /// The lowest latency recorded across all samples.
    pub min_latency_ms: Option<u64>,
    /// The arithmetic mean of all successful latency samples.
    pub avg_latency_ms: Option<u64>,
    /// The highest latency recorded across all samples.
    pub max_latency_ms: Option<u64>,
    /// A composite score from 0 (broken) to 100 (perfect) representing overall stability.
    ///
    /// Factors in jitter, packet loss, and latency spikes.
    pub stability_score: u8,
}

/// A high-level summary of the network's current state.
#[derive(Debug, Clone)]
pub struct NetworkStatus {
    /// True if the network is considered "connected" based on the [CheckStrategy].
    pub is_connected: bool,
    /// The categorical quality of the connection (e.g., Excellent, Poor, Offline).
    pub quality: ConnectionQuality,
    /// Detailed statistical breakdown of the connection's performance.
    pub latency_stats: LatencyStats,
    /// The label of the target that provided the fastest response in this check cycle.
    pub winner_target: String,
}

/// The comprehensive report produced by a network reachability check.
///
/// This structure aggregates high-level status, low-level metrics,
/// security findings, and individual target results.
#[derive(Debug, Clone)]
pub struct NetworkReport {
    /// The UTC timestamp (milliseconds since epoch) when the check began.
    pub timestamp_ms: u64,
    /// High-level connectivity status and quality assessment.
    pub status: NetworkStatus,
    /// The identified type of the active network interface (e.g., WiFi).
    pub connection_type: ConnectionType,
    /// Results of security-specific probes (VPN, DNS spoofing, etc.).
    pub security_flags_result: SecurityFlagsResult,
    /// An array containing the individual results for every target checked.
    pub target_reports: Vec<TargetReport>,
}
