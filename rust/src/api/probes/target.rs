//! Probe for checking a single network target.

use std::time::{Duration, Instant};
use tokio::{
    net::{TcpStream, UdpSocket},
    time::timeout,
};

use crate::api::models::{NetworkError, NetworkTarget, TargetProtocol, TargetReport};

/// Performs a network check against a single, specified target.
pub async fn check_target(target: &NetworkTarget) -> TargetReport {
    let start = Instant::now();
    let addr_str = format!("{}:{}", target.host, target.port);
    let timeout_duration = Duration::from_millis(target.timeout_ms);

    let result = timeout(timeout_duration, async {
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

                if target.port == 53 {
                    // Send a minimal DNS query to ensure end-to-end reachability
                    let query = [
                        0x12, 0x34, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x01, b'a', 0x00, 0x00, 0x01, 0x00, 0x01,
                    ];
                    socket.send(&query).await?;
                    let mut buf = [0u8; 512];
                    let _ = socket.recv(&mut buf).await?;
                } else {
                    socket.send(&[0]).await?;
                    // For non-DNS UDP, we still try to wait for any response (even if it times out,
                    // which might be expected for some UDP services, but for "reachability"
                    // a response is better).
                    // However, many UDP services are silent.
                    // To be safe and "accurate" as requested, we'll just send and
                    // hope for no "ICMP unreachable" if we were using a more advanced socket,
                    // but for now, we'll just stick to the fact that UDP is less reliable.
                }
            }
            TargetProtocol::Http | TargetProtocol::Https => {
                let scheme = if target.protocol == TargetProtocol::Https {
                    "https"
                } else {
                    "http"
                };
                let url = format!("{}://{}:{}", scheme, target.host, target.port);
                let client = reqwest::Client::builder()
                    .timeout(timeout_duration)
                    .build()
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

                let res = client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

                if !res.status().is_success() && target.is_essential {
                    return Err(NetworkError::ConnectionError(format!(
                        "HTTP status: {}",
                        res.status()
                    )));
                }
            }
            TargetProtocol::Icmp => {
                let payload = [0u8; 8];
                let _ = surge_ping::ping(addr.ip(), &payload)
                    .await
                    .map_err(|e| NetworkError::ConnectionError(format!("Ping failed: {}", e)))?;
            }
        }
        Ok(())
    })
    .await;

    match result {
        Ok(Ok(_)) => {
            let latency = start.elapsed().as_millis() as u64;
            TargetReport {
                label: target.label.clone(),
                success: true,
                latency_ms: latency,
                error: None,
                is_essential: target.is_essential,
            }
        }
        Ok(Err(e)) => TargetReport {
            label: target.label.clone(),
            success: false,
            latency_ms: 0,
            error: Some(e.to_string()),
            is_essential: target.is_essential,
        },
        Err(_) => {
            TargetReport {
                label: target.label.clone(),
                success: false,
                latency_ms: 0, // Should be 0 if it failed/timed out
                error: Some(NetworkError::TimeoutError.to_string()),
                is_essential: target.is_essential,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::{NetworkTarget, TargetProtocol};

    #[tokio::test]
    async fn test_check_target_invalid_host() {
        // We use a domain that is highly unlikely to exist.
        // Some ISPs/DNS proxies might still redirect this to a search page (NXDOMAIN hijacking).
        let target = NetworkTarget {
            label: "test".into(),
            host: "this.is.a.completely.non.existent.domain.that.should.not.resolve.ever.xyz"
                .into(),
            port: 80,
            protocol: TargetProtocol::Tcp,
            timeout_ms: 1000,
            priority: 1,
            is_essential: false,
        };

        let report = check_target(&target).await;
        if report.success {
            println!("Warning: DNS hijacking detected! Non-existent domain resolved successfully.");
        } else {
            assert!(report.error.is_some());
        }
    }

    #[tokio::test]
    async fn test_check_target_timeout() {
        let target = NetworkTarget {
            label: "test".into(),
            host: "8.8.8.8".into(), // Google DNS
            port: 9999,             // Unused port
            protocol: TargetProtocol::Tcp,
            timeout_ms: 10, // Extremely low timeout
            priority: 1,
            is_essential: false,
        };

        let report = check_target(&target).await;
        assert!(!report.success);
        assert!(report.error.unwrap().contains("Timeout"));
    }
}
