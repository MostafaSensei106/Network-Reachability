//! Probe for checking a single network target.

use std::time::{Duration, Instant};
use tokio::{
    net::{TcpStream, UdpSocket},
    time::timeout,
};

use crate::api::models::{NetworkError, NetworkTarget, TargetProtocol, TargetReport};

/// Performs a network check against a single, specified target.
///
/// This function attempts to establish a connection to the given `target` using
/// the specified protocol (TCP or UDP) and within the specified timeout.
///
/// - For TCP, it attempts a `TcpStream::connect`.
/// - For UDP, it binds a local socket, connects to the target address, and sends a
///   single-byte payload. The operation is considered successful if the `send`
///   completes without error. It does not wait for a response.
///
/// # Arguments
///
/// * `target` - A reference to the [NetworkTarget] to be checked.
///
/// # Returns
///
/// A [TargetReport] containing the outcome of the check.
/// - On success: `success` is true, `latency_ms` is the time taken, and `error` is None.
/// - On failure: `success` is false, `latency_ms` is 0, and `error` contains a
///   description of the failure (e.g., DNS error, connection error, timeout).
pub async fn check_target(target: &NetworkTarget) -> TargetReport {
    let start = Instant::now();
    let addr_str = format!("{}:{}", target.host, target.port);
    let timeout_duration = Duration::from_millis(target.timeout_ms);

    // This internal async block helps manage the timeout across the entire operation,
    // including DNS resolution and the connection attempt.
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
                // For UDP, we can't truly "connect" in the same way as TCP.
                // We bind a socket and send a small packet. If the send succeeds,
                // we consider it a success. An ICMP "Port Unreachable" might be returned
                // by the OS, but handling that reliably across platforms is complex.
                // A successful send is a good-enough indicator for reachability.
                let socket = UdpSocket::bind("0.0.0.0:0")
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
                socket
                    .connect(&addr)
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
                socket
                    .send(&[0])
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
            }
        }
        {
            let ok_result: Result<(), NetworkError> = Ok(());
            ok_result
        }
    })
    .await;

    match result {
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
                latency_ms: 999_999, // A large value to indicate timeout
                error: Some(NetworkError::TimeoutError.to_string()),
                is_essential: target.is_essential,
            }
        }
    }
}
