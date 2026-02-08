use crate::api::constants::AppConstants;
use crate::api::models::{
    CaptivePortalStatus, ConnectionQuality, ConnectionType, LocalDevice, NetworkMetadata,
    QualityThresholds, TraceHop,
};
use ipnet::IpNet;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};
use tokio::net::{TcpStream as TokioTcpStream, UdpSocket as TokioUdpSocket};
use tokio::time::timeout;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

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
    } else if latency <= threshold.great {
        ConnectionQuality::Great
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

pub async fn scan_local_network(
    subnet: String,
    scan_port: u16,
    timeout_ms: u64,
) -> Vec<LocalDevice> {
    let mut devices = Vec::new();
    let timeout_duration = Duration::from_millis(timeout_ms);

    let network: IpNet = match subnet.parse() {
        Ok(net) => net,
        Err(_) => {
            eprintln!("Invalid subnet format: {}", subnet);
            return devices;
        }
    };

    let mut join_handles = Vec::new();

    for host in network.hosts() {
        let ip_addr = host.to_string();
        let socket_addr: SocketAddr = format!("{}:{}", ip_addr, scan_port)
            .parse()
            .expect("Failed to parse socket address for local scan");
        let timeout_duration_clone = timeout_duration;

        let handle = tokio::spawn(async move {
            match timeout(
                timeout_duration_clone,
                TokioTcpStream::connect(&socket_addr),
            )
            .await
            {
                Ok(Ok(_)) => {
                    // Device found
                    Some(LocalDevice {
                        ip_address: ip_addr,
                        hostname: None, // Hostname resolution is complex and often slow. Skip for now.
                        mac_address: None, // MAC address requires ARP, platform-specific. Skip for now.
                    })
                }
                _ => None, // Connection error or timeout
            }
        });
        join_handles.push(handle);
    }

    for handle in join_handles {
        if let Ok(Some(device)) = handle.await {
            devices.push(device);
        }
    }

    devices
}

pub async fn check_for_captive_portal(timeout_ms: u64) -> CaptivePortalStatus {
    let client = match reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::limited(5)) // Follow up to 5 redirects
        .timeout(Duration::from_millis(timeout_ms))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to build reqwest client: {:?}", e);
            return CaptivePortalStatus {
                is_captive_portal: false,
                redirect_url: None,
            };
        }
    };

    let url = AppConstants::CAPTIVE_PORTAL_DETECTION_URL;

    match client.get(url).send().await {
        Ok(response) => {
            let final_url = response.url().to_string();
            let is_redirected = final_url != url;
            let status_code = response.status();

            // A common indicator of a captive portal is a redirect to a login page
            // or an unexpected non-200 status code from the original URL.
            // If there's a redirect, it's very likely a captive portal.
            // If the status is not 200 OK and it's not the original URL, also possible.
            let is_captive = is_redirected || !status_code.is_success();

            CaptivePortalStatus {
                is_captive_portal: is_captive,
                redirect_url: if is_redirected { Some(final_url) } else { None },
            }
        }
        Err(e) => {
            // If the request fails, it might also indicate a captive portal blocking access,
            // or simply no internet. For simplicity, we'll assume no captive portal if request completely fails.
            // A more robust check might differentiate between network error and captive portal blocking.
            eprintln!("Error checking for captive portal: {:?}", e);
            CaptivePortalStatus {
                is_captive_portal: false,
                redirect_url: None,
            }
        }
    }
}

pub async fn trace_route(host: String, max_hops: u8, timeout_per_hop_ms: u64) -> Vec<TraceHop> {
    let mut hops = Vec::new();
    let initial_resolver_config = ResolverConfig::default();
    let initial_resolver_opts = ResolverOpts::default();

    let target_ip: IpAddr = match tokio::task::spawn_blocking({
        let host_clone = host.clone(); // Clone host here for the blocking task
        let config_clone = initial_resolver_config.clone(); // Clone config for this blocking task
        let opts_clone = initial_resolver_opts.clone(); // Clone opts for this blocking task
        move || {
            let resolver = Resolver::new(config_clone, opts_clone)
                .expect("Failed to create DNS resolver inside blocking task");
            resolver.lookup_ip(host_clone.as_str())
        }
    })
    .await
    {
        Ok(Ok(response)) => {
            if let Some(ip) = response.iter().next() {
                ip
            } else {
                eprintln!("No IP address found for host {}", host); // `host` is still available here
                return hops;
            }
        }
        Ok(Err(e)) => {
            eprintln!("Failed to resolve host {}: {:?}", host, e); // `host` is still available here
            return hops;
        }
        Err(e) => {
            eprintln!("Blocking task failed for host {}: {:?}", host, e); // `host` is still available here
            return hops;
        }
    };

    let target_port: u16 = 33434; // Common traceroute UDP port
    let probe_payload = [0; 1]; // Small payload

    for ttl in 1..=max_hops {
        let socket = match TokioUdpSocket::bind("0.0.0.0:0").await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to bind UDP socket: {:?}", e);
                break;
            }
        };

        // Set TTL (Time To Live)
        if let Err(e) = socket.set_ttl(ttl as u32) {
            eprintln!("Failed to set TTL for hop {}: {:?}", ttl, e);
            break;
        }

        let destination_addr = SocketAddr::new(target_ip, target_port);
        let start_time = Instant::now();

        // Send UDP probe
        if let Err(e) = socket.send_to(&probe_payload, &destination_addr).await {
            eprintln!("Failed to send UDP probe for hop {}: {:?}", ttl, e);
            continue;
        }

        let mut buffer = [0; 2048];
        match timeout(
            Duration::from_millis(timeout_per_hop_ms),
            socket.recv_from(&mut buffer),
        )
        .await
        {
            Ok(Ok((_, src_addr))) => {
                let latency = start_time.elapsed().as_millis() as u64;
                let hop_ip = src_addr.ip().to_string();

                // Clone again for this blocking task
                let config_clone_for_reverse = initial_resolver_config.clone();
                let opts_clone_for_reverse = initial_resolver_opts.clone();
                let hostname = match tokio::task::spawn_blocking(move || {
                    let resolver = Resolver::new(config_clone_for_reverse, opts_clone_for_reverse)
                        .expect(
                            "Failed to create DNS resolver inside blocking task for reverse lookup",
                        );
                    resolver.reverse_lookup(src_addr.ip())
                })
                .await
                {
                    Ok(Ok(lookup)) => lookup.iter().next().map(|name| name.to_string()),
                    _ => None, // Handle errors from blocking task or lookup
                };

                hops.push(TraceHop {
                    hop_number: ttl,
                    ip_address: hop_ip,
                    hostname,
                    latency_ms: Some(latency),
                });

                if src_addr.ip() == target_ip {
                    // Reached destination
                    break;
                }
            }
            Ok(Err(e)) => {
                eprintln!("Error receiving for hop {}: {:?}", ttl, e);
                hops.push(TraceHop {
                    hop_number: ttl,
                    ip_address: "*".to_string(), // Indicate no response
                    hostname: None,
                    latency_ms: None,
                });
            }
            Err(_) => {
                // Timeout
                hops.push(TraceHop {
                    hop_number: ttl,
                    ip_address: "*".to_string(), // Indicate timeout
                    hostname: None,
                    latency_ms: None,
                });
            }
        }
    }
    hops
}
