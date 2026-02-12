//! Data structures for low-level network information and security flags.

/// Represents the type of physical or logical network connection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionType {
    /// A WiFi connection.
    Wifi,
    /// A mobile data connection (e.g., LTE, 5G).
    Cellular,
    /// A wired Ethernet connection.
    Ethernet,
    /// A Virtual Private Network connection.
    Vpn,
    /// A Bluetooth tethering connection.
    Bluetooth,
    /// The connection type could not be determined.
    Unknown,
}

/// The result of a captive portal check.
#[derive(Debug, Clone)]
pub struct CaptivePortalStatus {
    /// True if a captive portal was detected (i.e., the probe was redirected).
    pub is_captive_portal: bool,
    /// The URL that the probe was redirected to, if a portal was detected.
    pub redirect_url: Option<String>,
}

/// Represents a single hop in a traceroute path.
#[derive(Debug, Clone)]
pub struct TraceHop {
    /// The hop number in the sequence (Time-To-Live value).
    pub hop_number: u8,
    /// The IP address of the router at this hop. Can be "*" if the hop timed out.
    pub ip_address: String,
    /// The hostname resolved from the IP address, if available.
    pub hostname: Option<String>,
    /// The round-trip time to this hop in milliseconds.
    pub latency_ms: Option<u64>,
}

/// A report of security-related attributes of the current network connection.
#[derive(Debug, Clone)]
pub struct SecurityFlags {
    /// True if the active interface is a known VPN/tunnel type (e.g., 'tun', 'ppp').
    pub is_vpn_detected: bool,
    /// True if a DNS mismatch was found between system and trusted resolvers,
    /// indicating a potential DNS hijacking or spoofing attack.
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
