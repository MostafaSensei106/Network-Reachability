use ::std::time::{Duration, Instant};

use ::chrono::Utc;
use ::futures::future::join_all;
use ::tokio::{self, time::timeout};
use tokio::net::TcpStream; // For UDP send/receive

use crate::api::{
    models::{
        CheckStrategy, ConnectionQuality, NetworkError, NetworkReport, NetworkStatus,
        NetworkTarget, NetwrokConfiguration, TargetProtocol, TargetReport,
    },
    utils::{detect_network_metadata, evaluate_quality},
};

async fn check_target_internal(target: &NetworkTarget) -> TargetReport {
    let start = Instant::now();
    let addr_str = format!("{}:{}", target.host, target.port);
    let timeout_duration = Duration::from_millis(target.timeout_ms);

    let result: Result<(), NetworkError> = async {
        let mut addrs = tokio::net::lookup_host(&addr_str)
            .await
            .map_err(|e| NetworkError::DnsResolutionError(e.to_string()))?;
        let addr = addrs.next().ok_or(NetworkError::DnsResolutionError(
            "DNS Resolution failed".to_string(),
        ))?;

        match target.protocol {
            TargetProtocol::Tcp => {
                let _stream = TcpStream::connect(&addr)
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
                Ok(())
            }
            TargetProtocol::Udp => {
                let socket = tokio::net::UdpSocket::bind("0.0.0.0:0")
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
                socket
                    .connect(&addr)
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;
                // Send a small packet and try to receive a response to verify UDP connectivity
                let send_buf = [1; 1]; // Small test packet
                socket
                    .send(&send_buf)
                    .await
                    .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

                let mut recv_buf = [0; 1];
                let _ = timeout(Duration::from_millis(500), socket.recv(&mut recv_buf))
                    .await
                    .map_err(|_| NetworkError::TimeoutError)?; // Smaller timeout for UDP receive
                Ok(())
            }
        }
    }
    .await;

    match timeout(timeout_duration, async { result }).await {
        Ok(Ok(_)) => {
            let latency = start.elapsed().as_millis() as u64;
            TargetReport {
                label: target.label.clone(),
                success: true,
                latency_ms: Some(latency),
                error: None,
                is_essential: target.is_essential,
            }
        }
        Ok(Err(e)) => TargetReport {
            label: target.label.clone(),
            success: false,
            latency_ms: None,
            error: Some(e.to_string()),
            is_essential: target.is_essential,
        },
        Err(_) => TargetReport {
            label: target.label.clone(),
            success: false,
            latency_ms: None,
            error: Some(NetworkError::TimeoutError.to_string()),
            is_essential: target.is_essential,
        },
    }
}

// Helper function to calculate jitter metrics
fn _calculate_jitter(latencies: &[u64]) -> (Option<u64>, Option<u64>, Option<u64>, Option<f64>) {
    if latencies.is_empty() {
        return (None, None, None, None);
    }

    let min_latency = *latencies.iter().min().unwrap();
    let max_latency = *latencies.iter().max().unwrap();

    let sum: u64 = latencies.iter().sum();
    let count = latencies.len() as u64;
    let mean_latency = sum / count;

    if count < 2 {
        // Standard deviation requires at least 2 samples
        return (
            Some(min_latency),
            Some(max_latency),
            Some(mean_latency),
            None,
        );
    }

    let variance_sum: f64 = latencies
        .iter()
        .map(|&x| {
            let diff = (x as f64) - (mean_latency as f64);
            diff * diff
        })
        .sum();
    let std_dev = (variance_sum / (count as f64 - 1.0)).sqrt();

    (
        Some(min_latency),
        Some(max_latency),
        Some(mean_latency),
        Some(std_dev),
    )
}

pub async fn check_network(config: NetwrokConfiguration) -> NetworkReport {
    let start_time = Utc::now().timestamp_millis() as u64;

    let mut all_sample_latencies = Vec::new();
    let mut final_reports: Vec<TargetReport> = Vec::new();

    let num_samples = if config.num_jitter_samples == 0 {
        1
    } else {
        config.num_jitter_samples
    };

    for _ in 0..num_samples {
        let futures = config.targets.iter().map(|t| check_target_internal(t));
        let reports = join_all(futures).await;

        let mut best_latency_in_sample = u64::MAX;
        let mut success_count_in_sample = 0;
        let mut essential_failed_in_sample = false;

        for report in &reports {
            if report.is_essential && !report.success {
                essential_failed_in_sample = true;
            }

            if report.success {
                success_count_in_sample += 1;
                if let Some(lat) = report.latency_ms {
                    if lat < best_latency_in_sample {
                        best_latency_in_sample = lat;
                    }
                }
            }
        }

        let is_connected_in_sample = if essential_failed_in_sample {
            false
        } else {
            match config.check_strategy {
                CheckStrategy::Race => success_count_in_sample > 0,
                CheckStrategy::Consensus => {
                    let total = config.targets.len();
                    success_count_in_sample >= (total / 2) + 1
                }
            }
        };

        if is_connected_in_sample {
            all_sample_latencies.push(best_latency_in_sample);
        }

        // Only store the reports from the last sample to keep NetworkReport concise
        if config.num_jitter_samples == num_samples - 1 {
            final_reports = reports;
        }
    }

    let is_connected = !all_sample_latencies.is_empty();

    let (min_lat, max_lat, mean_lat, std_dev_lat) = _calculate_jitter(&all_sample_latencies);

    let final_latency = mean_lat.unwrap_or(0); // Use mean latency as the main latency if connected

    let mut quality = if is_connected {
        evaluate_quality(final_latency, &config.quality_threshold)
    } else {
        ConnectionQuality::Dead
    };

    // Check for unstable quality if jitter analysis is enabled and data is available
    if num_samples > 1 &&
       mean_lat.is_some() && mean_lat.unwrap() > 0 && // Avoid division by zero and check for valid mean
       std_dev_lat.is_some()
    {
        let mean_val = mean_lat.unwrap() as f64;
        let std_dev_val = std_dev_lat.unwrap();
        let jitter_threshold = mean_val * config.jitter_threshold_percent;

        if std_dev_val > jitter_threshold {
            quality = ConnectionQuality::Unstable;
        }
    }

    let (metadata, conn_type) = detect_network_metadata();

    NetworkReport {
        timestamp_ms: start_time,
        status: NetworkStatus {
            is_connected,
            quality,
            latency_ms: final_latency,
            winner_target: if is_connected {
                final_reports
                    .iter()
                    .find(|r| r.success)
                    .map_or("".to_string(), |r| r.label.clone())
            } else {
                "".to_string()
            }, // Placeholder, ideally this should be the winner of the final check or aggregated
            min_latency_ms: min_lat,
            max_latency_ms: max_lat,
            mean_latency_ms: mean_lat,
            std_dev_latency_ms: std_dev_lat,
        },
        connection_type: conn_type,
        metadata,
        target_reports: final_reports,
    }
}
