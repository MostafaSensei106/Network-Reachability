//! Probe for inspecting local network interfaces.

use crate::api::models::{ConnectionType, SecurityFlags};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

/// Inspects system network interfaces to detect connection type and potential security flags.
///
/// This function iterates through the system's available network interfaces,
/// skipping loopback and inactive ones. It identifies the most likely connection
/// type (VPN, WiFi, Ethernet, etc.) based on common interface name prefixes
/// (e.g., "tun", "wlan", "en").
///
/// A VPN connection is given the highest priority. If a VPN is detected, the
/// connection type will be [ConnectionType::Vpn] and the relevant security flag
/// will be set, regardless of other present interfaces.
///
/// # Returns
///
/// A tuple containing:
/// 1. `SecurityFlags` - A struct with flags like `is_vpn_detected` and the active `interface_name`.
/// 2. `ConnectionType` - The determined type of the network connection.
pub fn detect_security_and_network_type() -> (SecurityFlags, ConnectionType) {
    let interfaces = NetworkInterface::show().unwrap_or_default();

    let mut security_flags = SecurityFlags::default();
    let mut conn_type = ConnectionType::Unknown;

    // Keywords to identify different types of network interfaces.
    // Order matters: VPN check should be first.
    let type_map: &[(&[&str], ConnectionType)] = &[
        (&["tun", "tap", "ppp", "vpn"], ConnectionType::Vpn),
        (&["wlan", "wifi"], ConnectionType::Wifi),
        (&["eth", "en"], ConnectionType::Ethernet),
        (&["rmnet", "wwan"], ConnectionType::Cellular),
    ];

    // Find the active, non-loopback interface
    for iface in interfaces {
        // Skip loopback and interfaces without an IP
        if iface.name.contains("lo") || iface.addr.is_empty() {
            continue;
        }

        let name_lower = iface.name.to_lowercase();

        for &(keywords, ctype) in type_map {
            if keywords.iter().any(|kw| name_lower.contains(kw)) {
                if ctype == ConnectionType::Vpn {
                    // If a VPN is found, it's the most important piece of information.
                    // We set the flags and can break early.
                    security_flags.is_vpn_detected = true;
                    security_flags.interface_name = iface.name.clone();
                    conn_type = ConnectionType::Vpn;
                    break;
                } else if conn_type == ConnectionType::Unknown {
                    // Otherwise, set the first non-VPN type we find.
                    conn_type = ctype;
                    security_flags.interface_name = iface.name.clone();
                }
            }
        }

        // If VPN is detected, we don't need to check other interfaces.
        if security_flags.is_vpn_detected {
            break;
        }
    }

    (security_flags, conn_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_security_and_network_type_simple() {
        // This test is limited because it depends on the host's network interfaces.
        // A more robust test would use a mock library for `network_interface::show()`.
        let (flags, conn_type) = detect_security_and_network_type();

        // We can at least assert that it found *something*.
        assert!(!flags.interface_name.is_empty());
        assert_ne!(flags.interface_name, "unknown");
        // On CI runners, the connection type might be unknown, so we can't assert this.
        // assert_ne!(conn_type, ConnectionType::Unknown);
    }
}
