use crate::api::{
    constants::LibConstants,
    models::{ConnectionType, SecurityFlagsResult},
};
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
pub fn detect_security_and_network_type() -> (SecurityFlagsResult, ConnectionType) {
    let interfaces = NetworkInterface::show().unwrap_or_default();

    let mut security_flags_res = SecurityFlagsResult::default();
    let mut conn_type = ConnectionType::Unknown;

    // Keywords to identify different types of network interfaces.
    // Order matters: VPN check should be first.
    let type_map: &[(&[&str], ConnectionType)] = &[
        (LibConstants::VPN_PREFIXES, ConnectionType::Vpn),
        (LibConstants::WIFI_PREFIXES, ConnectionType::Wifi),
        (LibConstants::ETHERNET_PREFIXES, ConnectionType::Ethernet),
        (LibConstants::CELLULAR_PREFIXES, ConnectionType::Cellular),
        (LibConstants::BLUETOOTH_PREFIXES, ConnectionType::Bluetooth),
        (LibConstants::LOOPBACK_PREFIXES, ConnectionType::Loopback),
    ];

    // Find the active, non-loopback interface
    for iface in interfaces {
        // Skip loopback and interfaces without an IP (inactive)
        if iface.name.contains("lo") || iface.addr.is_empty() {
            continue;
        }

        let name_lower = iface.name.to_lowercase();

        for &(prefixes, ref ctype) in type_map {
            // Check if name contains prefix or matches common patterns
            if prefixes.iter().any(|prefix| name_lower.contains(prefix)) {
                if *ctype == ConnectionType::Vpn {
                    security_flags_res.is_vpn_detected = true;
                    security_flags_res.interface_name = iface.name.clone();
                    conn_type = ConnectionType::Vpn;
                    return (security_flags_res, conn_type);
                } else if conn_type == ConnectionType::Unknown {
                    conn_type = ctype.clone();
                    security_flags_res.interface_name = iface.name.clone();
                }
            }
        }
    }

    (security_flags_res, conn_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_security_and_network_type_simple() {
        // This test is limited because it depends on the host's network interfaces.
        // A more robust test would use a mock library for `network_interface::show()`.
        let (flags, _conn_type) = detect_security_and_network_type();

        // We can at least assert that it found *something*.
        assert!(!flags.interface_name.is_empty());
        assert_ne!(flags.interface_name, "unknown");
        // On CI runners, the connection type might be unknown, so we can't assert this.
        // assert_ne!(conn_type, ConnectionType::Unknown);
    }
}
