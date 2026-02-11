use crate::api::models::LocalDevice;
use ipnet::IpNet;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::time::timeout;

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
