use flutter_rust_bridge::frb;

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
pub struct CaptivePortalStatus {
    pub is_captive_portal: bool,
    pub redirect_url: Option<String>,
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

/// Comprehensive security report for the current network connection.
#[derive(Debug, Clone)]
pub struct SecurityFlags {
    /// True if the active interface is a known VPN/tunnel type (e.g., 'tun', 'ppp').
    pub is_vpn_detected: bool,
    /// True if a DNS mismatch was found between system and trusted resolvers.
    pub is_dns_spoofed: bool,
    /// True if a system-level proxy is detected (future implementation).
    pub is_proxy_detected: bool,
    /// The name of the active network interface (e.g., 'wlan0', 'tun0').
    pub interface_name: String,
}

impl Default for SecurityFlags {
    fn default() -> Self {
        Self {
            is_vpn_detected: false,
            is_dns_spoofed: false,
            is_proxy_detected: false,
            interface_name: "unknown".to_string(),
        }
    }
}
