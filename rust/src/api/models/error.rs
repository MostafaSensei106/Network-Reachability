//! Error types for the network reachability engine.
//!
//! This module defines the [`NetworkError`] enum, which categorizes various 
//! failure modes that can occur during network probing, such as DNS resolution 
//! issues, timeouts, and low-level connection errors.

/// Represents various types of network-related failures encountered during checks.
///
/// Each variant provides specific context about where and why the probe failed, 
/// allowing for precise diagnostic reporting and UI feedback.
#[derive(Debug, Clone)]
pub enum NetworkError {
    /// Failed to resolve the target hostname to an IP address.
    DnsResolutionError(String),

    /// A general connection failure at the transport or network layer.
    ConnectionError(String),

    /// The operation exceeded the allocated [`NetworkTarget::timeout_ms`].
    TimeoutError,

    /// An unexpected or unhandled error occurred within the engine.
    UnknownError(String),
}

impl std::fmt::Display for NetworkError {
    /// Formats the error for user-facing logs or UI display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError::DnsResolutionError(s) => write!(f, "DNS Resolution Error: {}", s),
            NetworkError::ConnectionError(s) => write!(f, "Connection Error: {}", s),
            NetworkError::TimeoutError => write!(f, "Timeout Error: Target failed to respond within the allotted time."),
            NetworkError::UnknownError(s) => write!(f, "Unknown Error: {}", s),
        }
    }
}

impl From<std::io::Error> for NetworkError {
    /// Automatically converts standard I/O errors into [`NetworkError::ConnectionError`].
    fn from(err: std::io::Error) -> Self {
        NetworkError::ConnectionError(err.to_string())
    }
}

impl From<anyhow::Error> for NetworkError {
    /// Automatically converts generic `anyhow` errors into [`NetworkError::UnknownError`].
    fn from(err: anyhow::Error) -> Self {
        NetworkError::UnknownError(err.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for NetworkError {
    /// Automatically converts Tokio timeout errors into [`NetworkError::TimeoutError`].
    fn from(_: tokio::time::error::Elapsed) -> Self {
        NetworkError::TimeoutError
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::io;

    async fn create_elapsed_error() -> tokio::time::error::Elapsed {
        tokio::time::timeout(tokio::time::Duration::from_millis(1), async {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
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
        assert!(NetworkError::TimeoutError.to_string().contains("Timeout Error"));
        assert_eq!(
            NetworkError::UnknownError("x".into()).to_string(),
            "Unknown Error: x"
        );
    }

    #[test]
    fn test_network_error_from_io_error() {
        let io_error = io::Error::other("test");
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
