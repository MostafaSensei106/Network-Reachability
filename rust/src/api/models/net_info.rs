//! Data structures for low-level network information and security flags.

use flutter_rust_bridge::frb;

use crate::api::constants::LibConstants;

/// Represents the physical or logical medium of the active network connection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionType {
    /// Connected via a wireless local area network (WiFi).
    Wifi,
    /// Connected via mobile data (e.g., 4G LTE, 5G NR).
    Cellular,
    /// Connected via a physical Ethernet cable.
    Ethernet,
    /// Connected through a Virtual Private Network (VPN) tunnel.
    Vpn,
    /// Connected via Bluetooth tethering.
    Bluetooth,
    /// A local loopback interface (typically 127.0.0.1).
    Loopback,
    /// The connection type could not be identified.
    Unknown,
}

impl Default for ConnectionType {
    /// Returns [ConnectionType::Unknown] as the fallback default.
    fn default() -> Self {
        return Self::Unknown;
    }
}

/// The result of a captive portal detection probe.
///
/// Captive portals are commonly found on public WiFi networks (e.g., cafes, airports)
/// where a login or "Terms of Service" acceptance is required before internet access is granted.
#[derive(Debug, Clone)]
pub struct CaptivePortalStatus {
    /// True if the probe was redirected, indicating a captive portal is intercepting traffic.
    pub is_captive_portal: bool,
    /// The destination URL of the redirection, if available.
    ///
    /// This is typically the login or landing page of the portal.
    pub redirect_url: Option<String>,
}

/// Represents a single diagnostic hop in a traceroute operation.
#[derive(Debug, Clone)]
pub struct TraceHop {
    /// The sequential hop number (starting from 1), corresponding to the Time-To-Live (TTL).
    pub hop_number: u8,
    /// The IP address of the router or gateway at this hop.
    ///
    /// Will be "*" if the hop failed to respond within the timeout.
    pub ip_address: String,
    /// The reverse-DNS hostname associated with the IP address, if it can be resolved.
    pub hostname: Option<String>,
    /// The measured round-trip time (RTT) to this hop in milliseconds.
    pub latency_ms: Option<u64>,
}

/// Internal representation of security-related attributes for the current connection.
#[derive(Debug, Clone)]
pub struct SecurityFlags {
    /// True if the active interface is identified as a VPN or tunnel (e.g., 'tun0', 'utun').
    pub is_vpn_detected: bool,
    /// True if DNS results differ significantly from trusted resolvers, suggesting tampering.
    pub is_dns_spoofed: bool,
    /// True if a system-level HTTP/HTTPS proxy is detected.
    pub is_proxy_detected: bool,
    /// The system-assigned name of the active network interface (e.g., 'en0', 'wlan0').
    pub interface_name: String,
}

/// The bridge-exported result containing security-related attributes.
///
/// This is an opaque type optimized for transfer across the FFI boundary.
#[frb(opaque)]
#[derive(Debug, Clone)]
pub struct SecurityFlagsResult {
    /// True if the active interface is identified as a VPN or tunnel (e.g., 'tun0', 'utun').
    pub is_vpn_detected: bool,
    /// True if DNS results differ significantly from trusted resolvers, suggesting tampering.
    pub is_dns_spoofed: bool,
    /// True if a system-level HTTP/HTTPS proxy is detected.
    pub is_proxy_detected: bool,
    /// The system-assigned name of the active network interface (e.g., 'en0', 'wlan0').
    pub interface_name: String,
}

/// Returns a "safe" default state:
/// - No VPN detected.
/// - No DNS spoofing detected.
/// - No proxy detected.
/// - Interface name set to a generic "unknown".
impl Default for SecurityFlagsResult {
    fn default() -> Self {
        Self {
            is_vpn_detected: false,
            is_dns_spoofed: false,
            is_proxy_detected: false,
            interface_name: LibConstants::DEFAULT_INTERFACE_NAME.to_string(),
        }
    }
}

/// Returns a "safe" default state:
/// - No VPN detected.
/// - No DNS spoofing detected.
/// - No proxy detected.
/// - Interface name set to a generic "unknown".
impl Default for SecurityFlags {
    fn default() -> Self {
        Self {
            is_vpn_detected: false,
            is_dns_spoofed: false,
            is_proxy_detected: false,
            interface_name: LibConstants::DEFAULT_INTERFACE_NAME.to_string(),
        }
    }
}
