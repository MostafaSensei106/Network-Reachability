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
    pub good: u64,
    pub moderate: u64,
    pub poor: u64,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            excellent: 50,
            good: 150,
            moderate: 300,
            poor: 1000,
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
pub struct Configuration {
    pub targets: Vec<NetworkTarget>,
    pub check_strategy: CheckStrategy,
    pub quality_threshold: QualityThresholds,
    pub check_interval_ms: u64,
    pub block_request_when_poor: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            targets: vec![
                NetworkTarget {
                    label: "Cloudflare".into(),
                    host: "1.1.1.1".into(),
                    port: 53,
                    protocol: TargetProtocol::Tcp,
                    timeout_ms: 1000,
                    priority: 1,
                    is_required: false,
                },
                NetworkTarget {
                    label: "Google".into(),
                    host: "8.8.8.8".into(),
                    port: 53,
                    protocol: TargetProtocol::Tcp,
                    timeout_ms: 1000,
                    priority: 1,
                    is_required: false,
                },
            ],
            check_strategy: CheckStrategy::Race,
            quality_threshold: QualityThresholds::default(),
            check_interval_ms: 5000,
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
