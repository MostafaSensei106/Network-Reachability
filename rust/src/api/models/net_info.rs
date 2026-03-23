//! Low-level network interface information and security diagnostics.
//!
//! This module provides the tools to identify *how* the device is connected 
//! (WiFi, Cellular, etc.) and perform security-sensitive checks like 
//! captive portal detection and DNS spoofing analysis.

use flutter_rust_bridge::frb;
use crate::api::constants::LibConstants;

/// Represents the physical or logical medium of the active network connection.
///
/// Understanding the `ConnectionType` is critical for bandwidth management 
/// (e.g., deferring large downloads on Cellular) and optimizing latency.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionType {
    /// Connected via a wireless local area network (802.11 WiFi).
    /// Typically implies higher bandwidth and lower cost than cellular.
    Wifi,

    /// Connected via a mobile carrier network (e.g., LTE, 5G, 3G).
    /// Usually implies metered data and potentially higher latency.
    Cellular,

    /// Connected via a physical RJ45 Ethernet cable.
    /// The most stable and high-performance connection type.
    Ethernet,

    /// Connected through a Virtual Private Network (VPN) tunnel.
    /// Indicates that traffic is being routed through an encrypted tunnel 
    /// which might mask the true underlying connection type (WiFi/Cellular).
    Vpn,

    /// Connected via Bluetooth tethering or a personal area network (PAN).
    /// Characterized by very low bandwidth and limited range.
    Bluetooth,

    /// A local loopback interface (typically 127.0.0.1).
    /// Indicates the device is communicating with itself; no external internet.
    Loopback,

    /// The connection type could not be identified by the OS or the engine.
    /// This is the fallback for unknown or experimental interface types.
    Unknown,
}

impl Default for ConnectionType {
    /// Returns [`ConnectionType::Unknown`] as the default state.
    fn default() -> Self {
        Self::Unknown
    }
}

/// The result of a captive portal detection probe.
///
/// Captive portals (intercepting gateways) are common in public spaces like 
/// airports and cafes. They appear as "connected" to the OS but block all 
/// non-authentication traffic.
#[derive(Debug, Clone)]
pub struct CaptivePortalStatus {
    /// True if the engine detected that HTTP requests are being redirected.
    ///
    /// If this is true, the `ConnectionQuality` will likely be set to 
    /// [`CaptivePortal`](super::config::ConnectionQuality::CaptivePortal).
    pub is_captive_portal: bool,

    /// The destination URL where the traffic was intercepted.
    ///
    /// If available, this is usually the login or "Terms of Service" page. 
    /// The application can use this to open a WebView for the user.
    pub redirect_url: Option<String>,
}

/// Represents a single diagnostic hop in a traceroute operation.
///
/// Traceroutes are used to map the path packets take from the device to 
/// a target, helping identify exactly where congestion or failure occurs.
#[derive(Debug, Clone)]
pub struct TraceHop {
    /// The sequential hop count (Time-To-Live value).
    /// Hop 1 is usually the local router/gateway.
    pub hop_number: u8,

    /// The IP address of the router at this hop.
    ///
    /// If a router at this hop does not respond to ICMP Time Exceeded messages, 
    /// this will be set to `"*"` (asterisk).
    pub ip_address: String,

    /// The reverse-DNS hostname of the hop's IP address.
    /// Only populated if a reverse lookup is successful.
    pub hostname: Option<String>,

    /// The Round-Trip Time (RTT) to this hop in milliseconds.
    /// `None` if the hop timed out.
    pub latency_ms: Option<u64>,
}

/// Internal representation of security-related attributes for the current connection.
///
/// This structure stores raw findings from the engine's security probes.
#[derive(Debug, Clone)]
pub struct SecurityFlags {
    /// True if the active network interface is identified as a tunnel/VPN.
    ///
    /// Detected by looking for interface names like `tun`, `tap`, `ppp`, or `utun`.
    pub is_vpn_detected: bool,

    /// True if the system DNS appears to be providing malicious or redirected results.
    ///
    /// Calculated by comparing local results for known static IPs against 
    /// trusted upstream resolvers.
    pub is_dns_spoofed: bool,

    /// True if a system-wide HTTP or SOCKS proxy is active.
    ///
    /// Proxies can intercept and modify application traffic.
    pub is_proxy_detected: bool,

    /// The system-assigned name of the active network interface.
    /// Examples: `wlan0` (Linux WiFi), `en0` (macOS WiFi), `eth0` (Ethernet).
    pub interface_name: String,
}

/// The bridge-exported result containing security-related attributes.
///
/// This version of the security flags is optimized for the FFI boundary 
/// and is marked as `opaque` for memory efficiency.
#[frb(opaque)]
#[derive(Debug, Clone)]
pub struct SecurityFlagsResult {
    /// Indicates if a VPN tunnel is active.
    pub is_vpn_detected: bool,
    /// Indicates if DNS tampering was detected.
    pub is_dns_spoofed: bool,
    /// Indicates if a proxy server is intercepting traffic.
    pub is_proxy_detected: bool,
    /// The name of the primary network interface (e.g., `en0`).
    pub interface_name: String,
}

impl Default for SecurityFlagsResult {
    /// Returns a "Safe/Clean" default state where no security issues are detected.
    fn default() -> Self {
        Self {
            is_vpn_detected: false,
            is_dns_spoofed: false,
            is_proxy_detected: false,
            interface_name: LibConstants::DEFAULT_INTERFACE_NAME.to_string(),
        }
    }
}

impl Default for SecurityFlags {
    /// Returns a "Safe/Clean" default state where no security issues are detected.
    fn default() -> Self {
        Self {
            is_vpn_detected: false,
            is_dns_spoofed: false,
            is_proxy_detected: false,
            interface_name: LibConstants::DEFAULT_INTERFACE_NAME.to_string(),
        }
    }
}
