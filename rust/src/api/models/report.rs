use super::config::ConnectionQuality;
use super::net_info::{ConnectionType, SecurityFlags};

#[derive(Debug, Clone)]
pub struct TargetReport {
    pub label: String,
    pub success: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
    pub is_essential: bool,
}

#[derive(Debug, Clone)]
pub struct LatencyStats {
    pub latency_ms: u64,
    pub jitter_ms: u64,
    pub packet_loss_percent: f32,
    pub min_latency_ms: Option<u64>,
    pub avg_latency_ms: Option<u64>,
    pub max_latency_ms: Option<u64>,
    pub stability_score: u8, // 0 - 100
}

/// A comprehensive snapshot of the network state at a given time.
#[derive(Debug, Clone)]
pub struct NetworkStatus {
    pub is_connected: bool,
    pub quality: ConnectionQuality,
    pub latency_stats: LatencyStats,
    pub winner_target: String,
    // pub latency_ms: u64,
    // pub jitter_ms: u64,
    // pub packet_loss_percent: f32,
    // pub winner_target: String,
    // pub min_latency_ms: Option<u64>,
    // pub max_latency_ms: Option<u64>,
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
