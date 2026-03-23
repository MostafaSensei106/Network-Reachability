//! Comprehensive data structures for network connectivity reports.
//!
//! This module defines the output of the reachability engine. A [`NetworkReport`] 
//! is the final "snapshot" containing high-level status, low-level metrics, 
//! security findings, and individual target results.

use crate::api::models::SecurityFlagsResult;
use super::config::ConnectionQuality;
use super::net_info::ConnectionType;

/// Detailed outcome of a connectivity check against a specific target.
///
/// Each configured target (e.g., Google DNS, Cloudflare) generates its 
/// own `TargetReport` during a check cycle.
#[derive(Debug, Clone)]
pub struct TargetReport {
    /// The unique label identifying the target (e.g., "Primary API Gateway").
    pub label: String,

    /// Indicates whether the target was successfully reached using the specified protocol.
    ///
    /// If `false`, the network is likely either down or this specific 
    /// target is experiencing an outage.
    pub success: bool,

    /// The measured latency in milliseconds for the fastest successful sample.
    ///
    /// This value is 0 if `success` is false.
    pub latency_ms: u64,

    /// A descriptive error message provided by the engine if the check failed.
    ///
    /// Common errors: "Connection Refused", "DNS Resolution Failed", "Timeout".
    pub error: Option<String>,

    /// Indicates if this target is critical for the overall "Connected" status.
    ///
    /// Failure of an essential target has a higher weight in the 
    /// [`CheckStrategy`](super::config::CheckStrategy).
    pub is_essential: bool,
}

/// A suite of statistical metrics derived from multiple latency samples.
///
/// These metrics provide deep insight into the "health" and consistency 
/// of the connection, helping identify subtle issues like bufferbloat 
/// or interference.
#[derive(Debug, Clone)]
pub struct LatencyStats {
    /// The representative latency value (usually the mean of all successful samples).
    /// Measured in milliseconds.
    pub latency_ms: u64,

    /// The "Jitter" value in milliseconds.
    ///
    /// Jitter is the variation in the delay of received packets (standard deviation).
    /// High jitter (e.g., > 30ms) causes "stuttering" in real-time applications 
    /// like VoIP, gaming, and video streaming.
    pub jitter_ms: u64,

    /// The percentage of samples (0.0 to 100.0) that failed to return a response.
    ///
    /// Even a small amount of packet loss (1-2%) can significantly degrade 
    /// TCP performance and causes noticeable lag in UDP-based services.
    pub packet_loss_percent: f32,

    /// The lowest latency recorded across all samples in the cycle.
    /// Represents the "best-case" performance of the current link.
    pub min_latency_ms: Option<u64>,

    /// The arithmetic mean of all successful latency samples.
    pub avg_latency_ms: Option<u64>,

    /// The highest latency recorded across all samples in the cycle.
    /// Often indicates transient congestion or "spikes".
    pub max_latency_ms: Option<u64>,

    /// A composite score from 0 (broken) to 100 (perfect) representing overall stability.
    ///
    /// The engine calculates this based on jitter consistency and packet loss. 
    /// Scores below 70 are typically flagged as [`ConnectionQuality::Unstable`].
    pub stability_score: u8,
}

/// A high-level summary of the network's current state.
///
/// This structure is what most UI layers will use to determine whether to 
/// show a "Connected" or "Offline" banner.
#[derive(Debug, Clone)]
pub struct NetworkStatus {
    /// True if the network is functionally "up" based on the configured strategy.
    pub is_connected: bool,

    /// The categorical quality of the connection (e.g., Excellent, Poor, Offline).
    ///
    /// Maps raw metrics to user-friendly buckets based on [`QualityThresholds`](super::config::QualityThresholds).
    pub quality: ConnectionQuality,

    /// Detailed statistical breakdown for performance analysis.
    pub latency_stats: LatencyStats,

    /// The label of the specific target that responded the fastest in this check.
    ///
    /// Useful for debugging and understanding which server is the 
    /// current "closest" endpoint.
    pub winner_target: String,
}

/// The comprehensive report produced by a network reachability check.
///
/// This is the final object returned by the engine after a manual check 
/// or emitted via a stream during periodic checks.
#[derive(Debug, Clone)]
pub struct NetworkReport {
    /// The UTC timestamp (milliseconds since epoch) when the check began.
    ///
    /// Used to track the "freshness" of the report and for historical analysis.
    pub timestamp_ms: u64,

    /// High-level connectivity status and quality assessment.
    pub status: NetworkStatus,

    /// The identified physical medium of the connection (WiFi, Cellular, etc.).
    pub connection_type: ConnectionType,

    /// Results of security-specific probes (VPN, DNS spoofing, etc.).
    pub security_flags_result: SecurityFlagsResult,

    /// An array containing the individual results for every target checked.
    ///
    /// Useful for granular debugging and displaying detailed per-server 
    /// status in a "Network Diagnostics" screen.
    pub target_reports: Vec<TargetReport>,
}
