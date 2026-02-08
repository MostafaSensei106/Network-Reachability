use crate::api::constants::AppConstants;
use flutter_rust_bridge::frb;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetProtocol {
    Tcp, // Connect (Reliable)
    Udp, // Send/Receive (Fast but tricky)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionQuality {
    Excellent,
    Great,
    Good,
    Moderate,
    Poor,
    Unstable,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionType {
    Wifi,
    Cellular,
    Ethernet,
    Vpn,
    Bluetooth,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct NetworkTarget {
    pub label: String,
    pub host: String,
    pub port: u16,
    pub protocol: TargetProtocol,
    pub timeout_ms: u64,
    pub priority: u8,       // 1 = High, 2 = Low
    pub is_essential: bool, // true = Essential, false = Optional
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckStrategy {
    Race,
    Consensus,
}

#[derive(Debug, Clone)]
pub struct NetworkStatus {
    pub is_connected: bool,
    pub quality: ConnectionQuality,
    pub latency_ms: u64,
    pub winner_target: String,
    pub min_latency_ms: Option<u64>,
    pub max_latency_ms: Option<u64>,
    pub mean_latency_ms: Option<u64>,
    pub std_dev_latency_ms: Option<f64>, // Use f64 for standard deviation
}

#[derive(Debug, Clone)]
pub struct NetworkMetadata {
    pub is_vpn: bool,
    pub interface_name: String,
}

#[derive(Debug, Clone)]
pub struct CaptivePortalStatus {
    pub is_captive_portal: bool,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NetwrokConfiguration {
    pub targets: Vec<NetworkTarget>,
    pub check_strategy: CheckStrategy,
    pub quality_threshold: QualityThresholds,
    pub check_interval_ms: u64,
    pub block_request_when_poor: bool,
    pub num_jitter_samples: u8, // Number of samples for jitter analysis
    pub jitter_threshold_percent: f64, // Percentage threshold for std dev to mark as unstable
}

impl Default for NetwrokConfiguration {
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
            check_strategy: CheckStrategy::Race,
            quality_threshold: QualityThresholds::default(),
            check_interval_ms: AppConstants::DEFAULT_CHECK_INTERVAL_MS,
            block_request_when_poor: false,
            num_jitter_samples: AppConstants::DEFAULT_JITTER_SAMPLES,
            jitter_threshold_percent: AppConstants::DEFAULT_JITTER_THRESHOLD_PERCENT,
        }
    }
}

#[frb(non_opaque)]
#[derive(Debug, Clone)]
pub struct LocalDevice {
    pub ip_address: String,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TraceHop {
    pub hop_number: u8,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct TargetReport {
    pub label: String,
    pub success: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>, // We'll keep this as string for now for simplicity in FFI
    pub is_essential: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkReport {
    pub timestamp_ms: u64,
    pub status: NetworkStatus,
    pub connection_type: ConnectionType,
    pub metadata: NetworkMetadata,
    pub target_reports: Vec<TargetReport>,
}

#[derive(Debug, Clone)]
pub enum NetworkError {
    DnsResolutionError(String),
    ConnectionError(String),
    TimeoutError,
    UnknownError(String),
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError::DnsResolutionError(s) => write!(f, "DNS Resolution Error: {}", s),
            NetworkError::ConnectionError(s) => write!(f, "Connection Error: {}", s),
            NetworkError::TimeoutError => write!(f, "Timeout Error"),
            NetworkError::UnknownError(s) => write!(f, "Unknown Error: {}", s),
        }
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> Self {
        NetworkError::ConnectionError(err.to_string())
    }
}

impl From<anyhow::Error> for NetworkError {
    fn from(err: anyhow::Error) -> Self {
        NetworkError::UnknownError(err.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for NetworkError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        NetworkError::TimeoutError
    }
}