use crate::api::constants::AppConstants;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetProtocol {
    Tcp, // Connect (Reliable)
    Udp, // Send/Receive (Fast but tricky)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionQuality {
    Excellent,
    Good,
    Moderate,
    Poor,
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
    pub priority: u8,      // 1 = High, 2 = Low
    pub is_required: bool, // true = Required, false = Optional
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
}

#[derive(Debug, Clone)]
pub struct NetworkMetadata {
    pub is_vpn: bool,
    pub interface_name: String,
}

#[derive(Debug, Clone)]
pub struct NetwrokConfiguration {
    pub targets: Vec<NetworkTarget>,
    pub check_strategy: CheckStrategy,
    pub quality_threshold: QualityThresholds,
    pub check_interval_ms: u64,
    pub block_request_when_poor: bool,
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
                    is_required: false,
                },
                NetworkTarget {
                    label: AppConstants::GOOGLE_NAME.into(),
                    host: AppConstants::GOOGLE_DNS.into(),
                    port: AppConstants::DEFAULT_PORT,
                    protocol: TargetProtocol::Tcp,
                    timeout_ms: AppConstants::DEFAULT_TIMEOUT_MS,
                    priority: 1,
                    is_required: false,
                },
            ],
            check_strategy: CheckStrategy::Race,
            quality_threshold: QualityThresholds::default(),
            check_interval_ms: AppConstants::DEFAULT_CHECK_INTERVAL_MS,
            block_request_when_poor: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TargetReport {
    pub label: String,
    pub success: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
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
