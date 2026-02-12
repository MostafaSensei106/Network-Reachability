//! Probe for tracing the network path to a host.

use crate::api::models::TraceHop;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};
use tokio::net::UdpSocket as TokioUdpSocket;
use tokio::task;
use tokio::time::timeout;
use trust_dns_resolver::system_conf::read_system_conf;
use trust_dns_resolver::Resolver;

/// Performs a traceroute to a specified host.
///
/// This function discovers the network path by sending a series of UDP packets
/// with incrementally increasing Time-To-Live (TTL) values. Each router along
/// the path decrements the TTL. When the TTL reaches zero, the router sends back
/// an ICMP "Time Exceeded" message. By capturing these messages, we can identify
/// each hop in the path.
///
/// # Arguments
///
/// * `host` - The destination hostname or IP address (e.g., "google.com").
/// * `max_hops` - The maximum number of hops (TTL value) to probe.
/// * `timeout_per_hop_ms` - The time in milliseconds to wait for a response from each hop.
///
/// # Returns
///
/// A `Vec<TraceHop>` where each element represents a router or the final
/// destination in the network path. Hops that time out will have an IP address
/// of "*" and no latency information.
pub async fn trace_route(host: String, max_hops: u8, timeout_per_hop_ms: u64) -> Vec<TraceHop> {
    let mut hops = Vec::new();

    // 1. Resolve the target host to an IP address first.
    let host_clone = host.clone();
    let target_ip_res = task::spawn_blocking(move || {
        let (config, opts) = read_system_conf().expect("Failed to read system DNS config");
        let resolver = Resolver::new(config, opts).expect("Failed to create system DNS resolver");
        resolver.lookup_ip(&host_clone)
    })
    .await;

    let target_ip: IpAddr = match target_ip_res {
        Ok(Ok(response)) => {
            if let Some(ip) = response.iter().next() {
                ip
            } else {
                eprintln!("No IP address found for host {}", host);
                return hops;
            }
        }
        _ => {
            eprintln!("Failed to resolve host {}", host);
            return hops;
        }
    };

    // A common UDP port used for traceroute that is unlikely to be used for a real service.
    let target_port: u16 = 33434;
    let probe_payload = [0; 1]; // Small, empty payload

    for ttl in 1..=max_hops {
        let socket = match TokioUdpSocket::bind("0.0.0.0:0").await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to bind UDP socket: {:?}", e);
                break;
            }
        };

        if let Err(e) = socket.set_ttl(ttl as u32) {
            eprintln!("Failed to set TTL for hop {}: {:?}", ttl, e);
            break;
        }

        let destination_addr = SocketAddr::new(target_ip, target_port);
        let start_time = Instant::now();

        if let Err(e) = socket.send_to(&probe_payload, &destination_addr).await {
            eprintln!("Failed to send UDP probe for hop {}: {:?}", ttl, e);
            continue;
        }

        // We need a buffer to receive the ICMP response
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

                hops.push(TraceHop {
                    hop_number: ttl,
                    ip_address: hop_ip,
                    hostname: None, // Reverse lookup is slow and often fails; can be added later if needed.
                    latency_ms: Some(latency),
                });

                // Stop if we've reached the final destination
                if src_addr.ip() == target_ip {
                    break;
                }
            }
            Ok(Err(_)) | Err(_) => {
                // This branch handles both read errors and timeouts.
                hops.push(TraceHop {
                    hop_number: ttl,
                    ip_address: "*".to_string(), // Standard representation for a timed-out hop
                    hostname: None,
                    latency_ms: None,
                });
            }
        }
    }
    hops
}
