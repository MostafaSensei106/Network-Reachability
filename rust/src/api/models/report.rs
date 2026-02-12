//! Data structures for the final, consolidated network check reports.

use super::config::ConnectionQuality;
use super::net_info::{ConnectionType, SecurityFlags};

/// The result of a check against a single network target.
#[derive(Debug, Clone)]
pub struct TargetReport {
    /// The unique label for the target.
    pub label: String,
    /// True if the check was successful.
    pub success: bool,
    /// The latency of the successful check in milliseconds.
    pub latency_ms: u64,
    /// An error message if the check failed.
    pub error: Option<String>,
    /// Whether this target is considered essential for the overall check.
    pub is_essential: bool,
}

/// A collection of statistical metrics for a series of latency samples.
#[derive(Debug, Clone)]
pub struct LatencyStats {
    /// The final, representative latency value, typically the mean.
    pub latency_ms: u64,
    /// The standard deviation of the latency samples, representing jitter.
    pub jitter_ms: u64,
    /// The percentage of failed samples out of the total expected.
    pub packet_loss_percent: f32,
    /// The minimum latency recorded in the sample set.
    pub min_latency_ms: Option<u64>,
    /// The average latency of the sample set.
    pub avg_latency_ms: Option<u64>,
    /// The maximum latency recorded in the sample set.
    pub max_latency_ms: Option<u64>,
    /// A calculated score from 0-100 representing connection stability,
    /// factoring in jitter, packet loss, and latency spikes.
    pub stability_score: u8,
}

/// A high-level summary of the network state at a given time.
#[derive(Debug, Clone)]
pub struct NetworkStatus {
    /// True if a connection to any target was successfully established.
    pub is_connected: bool,
    /// The overall calculated quality of the connection.
    pub quality: ConnectionQuality,
    /// Detailed statistics about latency and stability.
    pub latency_stats: LatencyStats,
    /// The label of the target that responded fastest in the final sample.
    pub winner_target: String,
}

/// The top-level report containing all information from a comprehensive network check.
#[derive(Debug, Clone)]
pub struct NetworkReport {
    /// The timestamp (in milliseconds since epoch) when the check was initiated.
    pub timestamp_ms: u64,
    /// The high-level status and quality summary of the network connection.
    pub status: NetworkStatus,
    /// The detected type of the active network connection (e.g., WiFi, Cellular).
    pub connection_type: ConnectionType,
    /// Security-related flags for the connection (e.g., VPN, DNS spoofing).
    pub security_flags: SecurityFlags,
    /// A list of detailed reports for each individual target that was checked.
    pub target_reports: Vec<TargetReport>,
}
