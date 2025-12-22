use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn check_real_internet(target_ip: String, time_out_ms: u64) -> bool {
    let addr: SocketAddr = match format!("{}:53", target_ip).parse() {
        Ok(address) => address,
        Err(_) => return false,
    };

    let connection = timeout(
        Duration::from_millis(time_out_ms),
        TcpStream::connect(&addr),
    )
    .await;

    match connection {
        Ok(Ok(_stream)) => true,
        _ => false,
    }
}
