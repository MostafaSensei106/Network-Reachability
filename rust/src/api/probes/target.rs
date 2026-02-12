use std::time::{Duration, Instant};
use tokio::{
    net::{TcpStream, UdpSocket},
    time::timeout,
};

use crate::api::models::{NetworkError, NetworkTarget, TargetProtocol, TargetReport};

/// Performs a network check against a single target.
pub async fn check_target(target: &NetworkTarget) -> TargetReport {
    let start = Instant::now();
    let addr_str = format!("{}:{}", target.host, target.port);
    let timeout_duration = Duration::from_millis(target.timeout_ms);

    // This internal async block helps manage the timeout across the entire operation.
    let result: Result<(), NetworkError> = async {
        let mut addrs = tokio::net::lookup_host(&addr_str)
            .await
            .map_err(|e| NetworkError::DnsResolutionError(e.to_string()))?;

        let addr = addrs.next().ok_or_else(|| {
            NetworkError::DnsResolutionError(
                "DNS resolution failed to return any addresses.".to_string(),
            )
        })?;

        match target.protocol {
            TargetProtocol::Tcp => {
                let _stream = TcpStream::connect(&addr)
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
            }
            TargetProtocol::Udp => {
                let socket = UdpSocket::bind("0.0.0.0:0")
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
                socket
                    .connect(&addr)
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

                // For UDP, we send a small packet. A successful send is a good sign,
                // but we don't wait for a reply as it's often not guaranteed (e.g., DNS servers).
                // A more advanced check could expect an ICMP port unreachable if the port is closed.
                let send_buf = [0u8; 1];
                socket
                    .send(&send_buf)
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
            }
        }
        Ok(())
    }
    .await;

    // Apply the overall timeout
    match timeout(timeout_duration, async { result }).await {
        Ok(Ok(_)) => {
            // The operation inside succeeded within the timeout
            let latency = start.elapsed().as_millis() as u64;
            TargetReport {
                label: target.label.clone(),
                success: true,
                latency_ms: latency,
                error: None,
                is_essential: target.is_essential,
            }
        }
        Ok(Err(e)) => {
            // The operation failed, but within the timeout
            TargetReport {
                label: target.label.clone(),
                success: false,
                latency_ms: 0,
                error: Some(e.to_string()),
                is_essential: target.is_essential,
            }
        }
        Err(_) => {
            // The operation timed out
            TargetReport {
                label: target.label.clone(),
                success: false,
                latency_ms: 9999999,
                error: Some(NetworkError::TimeoutError.to_string()),
                is_essential: target.is_essential,
            }
        }
    }
}
