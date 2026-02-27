//! Probe for checking a single network target.

use async_trait::async_trait;

use crate::api::models::{NetworkTarget, TargetProtocol, TargetReport};
use crate::api::probes::base::NetworkProbe;

/// Native implementation of network reachability checks.
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeProbe;

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl NetworkProbe for NativeProbe {
    async fn check(&self, target: &NetworkTarget) -> TargetReport {
        use tokio::{
            io::{AsyncReadExt, AsyncWriteExt},
            net::TcpStream,
            time::timeout,
        };
        use std::time::{Duration, Instant};
        use crate::api::models::NetworkError;

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
                    let mut stream = TcpStream::connect(&addr)
                        .await
                        .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

                    let probe = format!(
                        "HEAD / HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
                        target.host
                    );

                    stream.write_all(probe.as_bytes()).await.map_err(|e| {
                        NetworkError::ConnectionError(format!("Failed to send probe: {}", e))
                    })?;

                    let mut buf = [0u8; 1];
                    stream.read(&mut buf).await.map_err(|e| {
                        NetworkError::ConnectionError(format!(
                            "No response from target (possible local interception): {}",
                            e
                        ))
                    })?;
                }

                TargetProtocol::Http | TargetProtocol::Https => {
                    let scheme = if target.protocol == TargetProtocol::Https {
                        "https"
                    } else {
                        "http"
                    };
                    let url = format!("{}://{}:{}", scheme, target.host, target.port);
                    let client = reqwest::Client::builder()
                        .danger_accept_invalid_certs(true)
                        .timeout(timeout_duration)
                        .build()
                        .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

                    let res = client
                        .get(&url)
                        .send()
                        .await
                        .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

                    let status = res.status();
                    if status.is_server_error() && target.is_essential {
                        return Err(NetworkError::ConnectionError(format!(
                            "HTTP server error: {}",
                            status
                        )));
                    }

                    let _ = res.bytes().await.map_err(|e| {
                        NetworkError::ConnectionError(format!("Failed to read response body: {}", e))
                    })?;
                }

                TargetProtocol::Icmp => {
                    let payload = [0u8; 8];

                    let ping_result = surge_ping::ping(addr.ip(), &payload)
                        .await
                        .map_err(|e| NetworkError::ConnectionError(format!("Ping failed: {}", e)))?;

                    let (_packet, rtt) = ping_result;
                    let is_loopback = addr.ip().is_loopback();

                    if !is_loopback && rtt < Duration::from_micros(100) {
                        return Err(NetworkError::ConnectionError(
                            "Suspiciously low RTT - possible local interception".to_string(),
                        ));
                    }
                }
            }
            Ok::<(), NetworkError>(())
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
            Err(_) => TargetReport {
                label: target.label.clone(),
                success: false,
                latency_ms: 0,
                error: Some("Timeout Error".to_string()),
                is_essential: target.is_essential,
            },
        }
    }
}

/// Web-specific implementation using Fetch API.
#[cfg(target_arch = "wasm32")]
pub struct WebProbe;

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl NetworkProbe for WebProbe {
    async fn check(&self, target: &NetworkTarget) -> TargetReport {
        use web_sys::{Request, RequestInit, RequestMode, Window};
        use wasm_bindgen_futures::JsFuture;
        use js_sys::Date;

        let start = Date::now();

        // Map protocols to http/https for Fetch
        let scheme = match target.protocol {
            TargetProtocol::Https => "https",
            _ => "http", // Fallback to http for ICMP/TCP as we can't do raw sockets in browser
        };

        let url = format!("{}://{}:{}", scheme, target.host, target.port);

        let opts = RequestInit::new();
        opts.set_method("HEAD");
        opts.set_mode(RequestMode::NoCors);

        let window: Window = web_sys::window().expect("Window not found");

        let request = match Request::new_with_str_and_init(&url, &opts) {
            Ok(req) => req,
            Err(e) => return TargetReport {
                label: target.label.clone(),
                success: false,
                latency_ms: 0,
                error: Some(format!("Failed to create request: {:?}", e)),
                is_essential: target.is_essential,
            },
        };

        let fetch_promise = window.fetch_with_request(&request);
        let result = JsFuture::from(fetch_promise).await;

        let end = Date::now();
        let latency = (end - start) as u64;

        match result {
            Ok(_) => {
                TargetReport {
                    label: target.label.clone(),
                    success: true,
                    latency_ms: latency,
                    error: None,
                    is_essential: target.is_essential,
                }
            }
            Err(e) => {
                TargetReport {
                    label: target.label.clone(),
                    success: false,
                    latency_ms: 0,
                    error: Some(format!("Fetch failed: {:?}", e)),
                    is_essential: target.is_essential,
                }
            }
        }
    }
}

/// Performs a network check against a single, specified target.
pub async fn check_target(target: &NetworkTarget) -> TargetReport {
    #[cfg(not(target_arch = "wasm32"))]
    {
        NativeProbe.check(target).await
    }
    #[cfg(target_arch = "wasm32")]
    {
        WebProbe.check(target).await
    }
}
