use super::config::ConnectionQuality;
use super::net_info::{ConnectionType, SecurityFlags};

#[derive(Debug, Clone)]
pub struct TargetReport {
    pub label: String,
    pub success: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>, // We'll keep this as string for now for simplicity in FFI
    pub is_essential: bool,
}

/// A comprehensive snapshot of the network state at a given time.
#[derive(Debug, Clone)]
pub struct NetworkStatus {
    /// The final, overall result: true if the network is usable.
    pub is_connected: bool,
    /// The evaluated quality of the connection (Excellent, Good, Poor, etc.).
    pub quality: ConnectionQuality,
    /// The mean latency in milliseconds from the winning target(s).
    pub latency_ms: u64,
    /// The standard deviation of latency in milliseconds, indicating stability.
    pub jitter_ms: u64,
    /// The percentage of packets lost during the check (future implementation).
    pub packet_loss_percent: f32,
    /// The label of the target that responded first in a 'Race' strategy.
    pub winner_target: String,
    /// Minimum latency recorded during jitter analysis.
    pub min_latency_ms: Option<u64>,
    /// Maximum latency recorded during jitter analysis.
    pub max_latency_ms: Option<u64>,
}

/// The top-level report containing all information about a network check.
#[derive(Debug, Clone)]
pub struct NetworkReport {
    /// The timestamp (in milliseconds since epoch) when the check was initiated.
    pub timestamp_ms: u64,
    /// The detailed status of the network connection.
    pub status: NetworkStatus,
    /// The detected type of the active network connection (WiFi, Cellular, etc.).
    pub connection_type: ConnectionType,
    /// Security-related flags for the connection.
    pub security_flags: SecurityFlags,
    /// A list of reports for each individual target that was checked.
    pub target_reports: Vec<TargetReport>,
}
