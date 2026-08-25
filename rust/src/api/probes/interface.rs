use crate::api::models::{ConnectionType, SecurityFlagsResult};

/// Inspects system network interfaces to detect connection type and potential security flags.
#[cfg(not(target_arch = "wasm32"))]
pub fn detect_security_and_network_type() -> (SecurityFlagsResult, ConnectionType) {
    use crate::api::constants::LibConstants;
    use network_interface::{NetworkInterface, NetworkInterfaceConfig};

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

        for &(prefixes, ref ctype) in type_map {
            // Case-insensitive prefix check without allocating a new String
            let name_matches = prefixes.iter().any(|prefix| {
                iface.name.len() >= prefix.len()
                    && iface.name.as_bytes().iter().zip(prefix.as_bytes()).all(
                        |(&a, &b)| a.to_ascii_lowercase() == b,
                    )
                    || iface
                        .name
                        .as_bytes()
                        .windows(prefix.len())
                        .any(|window| {
                            window
                                .iter()
                                .zip(prefix.as_bytes())
                                .all(|(&a, &b)| a.to_ascii_lowercase() == b)
                        })
            });
            if name_matches {
                if *ctype == ConnectionType::Vpn {
                    security_flags_res.is_vpn_detected = true;
                    security_flags_res.interface_name = iface.name.clone();
                    conn_type = ConnectionType::Vpn;
                    return (security_flags_res, conn_type);
                } else if conn_type == ConnectionType::Unknown {
                    conn_type = *ctype;
                    security_flags_res.interface_name = iface.name.clone();
                }
            }
        }
    }

    (security_flags_res, conn_type)
}

#[cfg(target_arch = "wasm32")]
pub fn detect_security_and_network_type() -> (SecurityFlagsResult, ConnectionType) {
    (SecurityFlagsResult::default(), ConnectionType::Unknown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_security_and_network_type_simple() {
        let (flags, _conn_type) = detect_security_and_network_type();
        assert!(!flags.interface_name.is_empty());
        assert_ne!(flags.interface_name, "unknown");
    }
}
