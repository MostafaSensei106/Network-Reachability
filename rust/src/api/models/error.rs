#[derive(Debug, Clone)]
pub enum NetworkError {
    DnsResolutionError(String),
    ConnectionError(String),
    TimeoutError,
    UnknownError(String),
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError::DnsResolutionError(s) => write!(f, "DNS Resolution Error: {}", s),
            NetworkError::ConnectionError(s) => write!(f, "Connection Error: {}", s),
            NetworkError::TimeoutError => write!(f, "Timeout Error"),
            NetworkError::UnknownError(s) => write!(f, "Unknown Error: {}", s),
        }
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> Self {
        NetworkError::ConnectionError(err.to_string())
    }
}

impl From<anyhow::Error> for NetworkError {
    fn from(err: anyhow::Error) -> Self {
        NetworkError::UnknownError(err.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for NetworkError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        NetworkError::TimeoutError
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::io;
    use tokio::time::{self, error::Elapsed};

    async fn create_elapsed_error() -> Elapsed {
        time::timeout(time::Duration::from_millis(1), async {
            time::sleep(time::Duration::from_millis(10)).await;
        })
        .await
        .unwrap_err()
    }

    #[test]
    fn test_network_error_display() {
        assert_eq!(
            NetworkError::DnsResolutionError("x".into()).to_string(),
            "DNS Resolution Error: x"
        );
        assert_eq!(
            NetworkError::ConnectionError("x".into()).to_string(),
            "Connection Error: x"
        );
        assert_eq!(NetworkError::TimeoutError.to_string(), "Timeout Error");
        assert_eq!(
            NetworkError::UnknownError("x".into()).to_string(),
            "Unknown Error: x"
        );
    }

    #[test]
    fn test_network_error_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::Other, "test");
        let network_error: NetworkError = io_error.into();
        assert!(matches!(network_error, NetworkError::ConnectionError(_)));
    }

    #[test]
    fn test_network_error_from_anyhow_error() {
        let err = anyhow!("boom");
        let network_error: NetworkError = err.into();
        assert!(matches!(network_error, NetworkError::UnknownError(_)));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_network_error_from_tokio_elapsed() {
        let elapsed_error = create_elapsed_error().await;
        let network_error: NetworkError = elapsed_error.into();
        assert!(matches!(network_error, NetworkError::TimeoutError));
    }
}
