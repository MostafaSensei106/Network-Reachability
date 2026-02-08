use ::std::{
    net::TcpStream,
    time::{Duration, Instant},
};

use ::chrono::Utc;
use ::futures::future::join_all;
use ::tokio::{self, time::timeout};

use crate::api::{
    models::{
        CheckStrategy, ConnectionQuality, NetworkReport, NetworkStatus, NetworkTarget,
        NetwrokConfiguration, TargetProtocol, TargetReport,
    },
    utils::{detect_network_metadata, evaluate_quality},
};

async fn check_target_internal(target: &NetworkTarget) -> TargetReport {
    let start = Instant::now();
    let addr_str = format!("{}:{}", target.host, target.port);
    let timeout_duration = Duration::from_millis(target.timeout_ms);

    let result: Result<(), ErrorType> = async {
        // DNS Lookup
        let mut addrs = tokio::net::lookup_host(&addr_str).await?;
        let addr = addrs
            .next()
            .ok_or(anyhow::anyhow!("DNS Resolution failed"))?;

        match target.protocol {
            TargetProtocol::Tcp => {
                let _stream = TcpStream::connect(&addr);
                Ok(())
            }
            TargetProtocol::Udp => {
                let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
                socket.connect(&addr).await?;
                // socket.send(&[0]).await?;
                Ok(())
            }
        }
    };

    match timeout(timeout_duration, result).await {
        Ok(Ok(_)) => {
            let latency = start.elapsed().as_millis() as u64;
            TargetReport {
                label: target.label.clone(),
                success: true,
                latency_ms: Some(latency),
                error: None,
                is_essential: target.is_required,
            }
        }
        Ok(Err(e)) => TargetReport {
            label: target.label.clone(),
            success: false,
            latency_ms: None,
            error: Some(e),
            is_essential: target.is_required,
        },
        Err(_) => TargetReport {
            label: target.label.clone(),
            success: false,
            latency_ms: None,
            error: Some("Timeout".to_string()),
            is_essential: target.is_required,
        },
    }
}

pub async fn check_network(config: NetwrokConfiguration) -> NetworkReport {
    let start_time = Utc::now().timestamp_millis() as u64;

    let futures = config.targets.iter().map(|t| check_target_internal(t));
    let reports = join_all(futures).await;

    let mut best_latency = u64::MAX;
    let mut winner = "None".to_string();
    let mut success_count = 0;
    let mut essential_failed = false;

    for report in &reports {
        if report.is_essential && !report.success {
            essential_failed = true;
        }

        if report.success {
            success_count += 1;
            if let Some(lat) = report.latency_ms {
                if lat < best_latency {
                    best_latency = lat;
                    winner = report.label.clone();
                }
            }
        }
    }

    let is_connected = if essential_failed {
        false
    } else {
        match config.check_strategy {
            CheckStrategy::Race => success_count > 0,
            CheckStrategy::Consensus => {
                let total = config.targets.len();
                success_count >= (total / 2) + 1
            }
        }
    };

    let final_latency = if is_connected { best_latency } else { 0 };
    let quality = if is_connected {
        evaluate_quality(final_latency, &config.quality_threshold)
    } else {
        ConnectionQuality::Dead
    };

    let (metadata, conn_type) = detect_network_metadata();

    NetworkReport {
        timestamp_ms: start_time,
        status: NetworkStatus {
            is_connected,
            quality,
            latency_ms: final_latency,
            winner_target: winner,
        },
        connection_type: conn_type,
        metadata,
        target_reports: reports,
    }
}
