use crate::api::models::{ConnectionQuality, ConnectionType, NetworkMetadata, QualityThresholds};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub fn detect_network_metadata() -> (NetworkMetadata, ConnectionType) {
    let interfaces = NetworkInterface::show().unwrap_or_default();

    let mut is_vpn = false;
    let mut conn_type = ConnectionType::Unknown;
    let mut active_interface_name = "unknown".to_string();

    let type_map: &[(&[&str], ConnectionType)] = &[
        (&["tun", "tap", "ppp", "vpn"], ConnectionType::Vpn),
        (&["wlan", "wifi"], ConnectionType::Wifi),
        (&["eth", "en"], ConnectionType::Ethernet),
        (&["rmnet"], ConnectionType::Cellular),
    ];

    for iface in interfaces {
        if iface.name.contains("lo") || iface.addr.is_empty() {
            continue;
        }

        let name_lower = iface.name.to_lowercase();

        for &(keywords, ctype) in type_map {
            if keywords.iter().any(|kw| name_lower.contains(kw)) {
                if ctype == ConnectionType::Vpn {
                    is_vpn = true;
                    conn_type = ConnectionType::Vpn;
                    active_interface_name = iface.name.clone();
                    break;
                } else if conn_type == ConnectionType::Unknown {
                    conn_type = ctype;
                    active_interface_name = iface.name.clone();
                    break;
                }
            }
        }

        if is_vpn {
            break;
        }
    }
    (
        NetworkMetadata {
            is_vpn,
            interface_name: active_interface_name,
        },
        conn_type,
    )
}

pub fn evaluate_quality(latency: u64, threshold: &QualityThresholds) -> ConnectionQuality {
    if latency <= threshold.excellent {
        ConnectionQuality::Excellent
    } else if latency <= threshold.good {
        ConnectionQuality::Good
    } else if latency <= threshold.moderate {
        ConnectionQuality::Moderate
    } else if latency <= threshold.poor {
        ConnectionQuality::Poor
    } else {
        ConnectionQuality::Dead
    }
}
