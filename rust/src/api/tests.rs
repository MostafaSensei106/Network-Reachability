#[cfg(test)]
mod tests {
    use super::super::constants::AppConstants;
    use super::super::engine::_calculate_jitter;
    use super::super::models::{
        ConnectionQuality, NetworkError, NetwrokConfiguration, QualityThresholds,
    };
    use super::super::utils::{check_for_captive_portal, evaluate_quality};
    use anyhow::anyhow;
    use mockito;
    use std::io;
    use tokio::time::{self, error::Elapsed};

    // Helper to create an Elapsed error for testing
    async fn create_elapsed_error() -> Elapsed {
        time::timeout(time::Duration::from_millis(1), async {
            time::sleep(time::Duration::from_millis(100)).await;
        })
        .await
        .unwrap_err()
    }

    #[test]
    fn test_evaluate_quality() {
        let thresholds = QualityThresholds::default();

        // Test excellent
        assert_eq!(
            evaluate_quality(thresholds.excellent - 1, &thresholds),
            ConnectionQuality::Excellent
        );
        assert_eq!(
            evaluate_quality(thresholds.excellent, &thresholds),
            ConnectionQuality::Excellent
        );

        // Test great
        assert_eq!(
            evaluate_quality(thresholds.great - 1, &thresholds),
            ConnectionQuality::Great
        );
        assert_eq!(
            evaluate_quality(thresholds.great, &thresholds),
            ConnectionQuality::Great
        );

        // Test good
        assert_eq!(
            evaluate_quality(thresholds.good - 1, &thresholds),
            ConnectionQuality::Good
        );
        assert_eq!(
            evaluate_quality(thresholds.good, &thresholds),
            ConnectionQuality::Good
        );

        // Test moderate
        assert_eq!(
            evaluate_quality(thresholds.moderate - 1, &thresholds),
            ConnectionQuality::Moderate
        );
        assert_eq!(
            evaluate_quality(thresholds.moderate, &thresholds),
            ConnectionQuality::Moderate
        );

        // Test poor
        assert_eq!(
            evaluate_quality(thresholds.poor - 1, &thresholds),
            ConnectionQuality::Poor
        );
        assert_eq!(
            evaluate_quality(thresholds.poor, &thresholds),
            ConnectionQuality::Poor
        );

        // Test dead
        assert_eq!(
            evaluate_quality(thresholds.poor + 1, &thresholds),
            ConnectionQuality::Dead
        );
    }

    #[test]
    fn test_calculate_jitter() {
        // Empty latencies
        let (min, max, mean, std_dev): (Option<u64>, Option<u64>, Option<u64>, Option<f64>) =
            _calculate_jitter(&[]);
        assert_eq!(min, None);
        assert_eq!(max, None);
        assert_eq!(mean, None);
        assert_eq!(std_dev, None);

        // Single latency
        let (min, max, mean, std_dev): (Option<u64>, Option<u64>, Option<u64>, Option<f64>) =
            _calculate_jitter(&[100]);
        assert_eq!(min, Some(100));
        assert_eq!(max, Some(100));
        assert_eq!(mean, Some(100));
        assert_eq!(std_dev, None); // Std dev requires at least 2 samples

        // Multiple latencies
        let latencies = vec![100, 110, 90, 105, 95];
        let (min, max, mean, std_dev): (Option<u64>, Option<u64>, Option<u64>, Option<f64>) =
            _calculate_jitter(&latencies);
        assert_eq!(min, Some(90));
        assert_eq!(max, Some(110));
        assert_eq!(mean, Some(100));
        assert!(std_dev.is_some());
        // Exact value for std_dev is 7.905694150420948, allow for float comparison
        assert!((std_dev.unwrap() - 7.905).abs() < 0.01);

        // All same latencies
        let latencies = vec![50, 50, 50, 50];
        let (min, max, mean, std_dev): (Option<u64>, Option<u64>, Option<u64>, Option<f64>) =
            _calculate_jitter(&latencies);
        assert_eq!(min, Some(50));
        assert_eq!(max, Some(50));
        assert_eq!(mean, Some(50));
        assert!(std_dev.is_some());
        assert!((std_dev.unwrap() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_network_configuration_default() {
        let config = NetwrokConfiguration::default();

        assert_eq!(config.targets.len(), 2);
        assert_eq!(config.targets[0].label, AppConstants::CLOUDFLARE_NAME);
        assert_eq!(config.targets[0].host, AppConstants::CLOUDFLARE_DNS);
        assert_eq!(config.targets[1].label, AppConstants::GOOGLE_NAME);
        assert_eq!(config.targets[1].host, AppConstants::GOOGLE_DNS);
        assert_eq!(
            config.check_interval_ms,
            AppConstants::DEFAULT_CHECK_INTERVAL_MS
        );
        assert_eq!(
            config.num_jitter_samples,
            AppConstants::DEFAULT_JITTER_SAMPLES
        );
        assert_eq!(
            config.jitter_threshold_percent,
            AppConstants::DEFAULT_JITTER_THRESHOLD_PERCENT
        );
    }

    #[test]
    fn test_quality_thresholds_default() {
        let thresholds = QualityThresholds::default();

        assert_eq!(
            thresholds.excellent,
            AppConstants::DEFAULT_EXCELLENT_THRESHOLD
        );
        assert_eq!(thresholds.great, AppConstants::DEFAULT_GREAT_THRESHOLD);
        assert_eq!(thresholds.good, AppConstants::DEFAULT_GOOD_THRESHOLD);
        assert_eq!(
            thresholds.moderate,
            AppConstants::DEFAULT_MODERATE_THRESHOLD
        );
        assert_eq!(thresholds.poor, AppConstants::DEFAULT_POOR_THRESHOLD);
    }

    #[test]
    fn test_network_error_display() {
        assert_eq!(
            NetworkError::DnsResolutionError("test dns".to_string()).to_string(),
            "DNS Resolution Error: test dns"
        );
        assert_eq!(
            NetworkError::ConnectionError("test connection".to_string()).to_string(),
            "Connection Error: test connection"
        );
        assert_eq!(NetworkError::TimeoutError.to_string(), "Timeout Error");
        assert_eq!(
            NetworkError::UnknownError("test unknown".to_string()).to_string(),
            "Unknown Error: test unknown"
        );
    }

    #[test]
    fn test_network_error_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::Other, "test io error");
        let network_error: NetworkError = io_error.into();
        assert_eq!(network_error.to_string(), "Connection Error: test io error");
    }

    #[test]
    fn test_network_error_from_anyhow_error() {
        let anyhow_error = anyhow!("test anyhow error");
        let network_error: NetworkError = anyhow_error.into();
        assert_eq!(
            network_error.to_string(),
            "Unknown Error: test anyhow error"
        );
    }

    #[tokio::test]
    async fn test_network_error_from_tokio_elapsed() {
        let elapsed_error = create_elapsed_error().await;
        let network_error: NetworkError = elapsed_error.into();
        assert_eq!(network_error.to_string(), "Timeout Error");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_check_for_captive_portal_no_redirect() {
        let mut server = mockito::Server::new();
        let _m = server
            .mock("GET", AppConstants::CAPTIVE_PORTAL_DETECTION_URL)
            .with_status(200)
            .create();

        let status = check_for_captive_portal(5000).await;
        assert!(!status.is_captive_portal);
        assert!(status.redirect_url.is_none());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_check_for_captive_portal_with_redirect() {
        let redirect_url = "http://captive.portal.com/login".to_string();
        let mut server = mockito::Server::new();
        let _m = server
            .mock("GET", AppConstants::CAPTIVE_PORTAL_DETECTION_URL)
            .with_status(302)
            .with_header("Location", &redirect_url)
            .create();

        let status = check_for_captive_portal(5000).await;
        assert!(status.is_captive_portal);
        assert_eq!(status.redirect_url, Some(redirect_url));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_check_for_captive_portal_non_200_status() {
        let mut server = mockito::Server::new();
        let _m = server
            .mock("GET", AppConstants::CAPTIVE_PORTAL_DETECTION_URL)
            .with_status(403)
            .create();

        let status = check_for_captive_portal(5000).await;
        assert!(!status.is_captive_portal);
        assert!(status.redirect_url.is_none());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_check_for_captive_portal_connection_error() {
        // Mock a connection error by not creating a mock, so the request fails
        let mut server = mockito::Server::new(); // Need server to get a valid URL, but no mock for connection error
        let status = check_for_captive_portal(100).await; // Use a short timeout
        assert!(!status.is_captive_portal);
        assert!(status.redirect_url.is_none());
    }

    // TODO: The following functions involve significant network I/O and are
    // difficult to unit test directly without advanced mocking or refactoring
    // to allow for dependency injection. They are better suited for
    // integration tests.
    // - engine::check_network
    // - engine::check_target_internal
    // - utils::detect_network_metadata
    // - utils::scan_local_network
    // - utils::trace_route
}
