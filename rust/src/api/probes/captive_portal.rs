//! Probe for detecting captive portals.

use crate::api::{constants::LibConstants, models::CaptivePortalStatus};

/// Checks for the presence of a captive portal.
pub async fn check_for_captive_portal(timeout_ms: u64) -> CaptivePortalStatus {
    use std::time::Duration;
    let client = match reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::limited(5)) // Follow up to 5 redirects
        .timeout(Duration::from_millis(timeout_ms))
        .build()
    {
        Ok(c) => c,
        Err(_) => {
            return CaptivePortalStatus {
                is_captive_portal: false,
                redirect_url: None,
            };
        }
    };

    let url = LibConstants::CAPTIVE_PORTAL_DETECTION_URL;

    match client.get(url).send().await {
        Ok(response) => {
            let final_url = response.url().to_string();
            let is_redirected = final_url != url;

            CaptivePortalStatus {
                is_captive_portal: is_redirected,
                redirect_url: if is_redirected { Some(final_url) } else { None },
            }
        }
        Err(_) => CaptivePortalStatus {
            is_captive_portal: false,
            redirect_url: None,
        },
    }
}

/// Web implementation stub (WASM removed).
pub async fn check_for_captive_portal_web_manual(_timeout_ms: u64) -> CaptivePortalStatus {
    CaptivePortalStatus {
        is_captive_portal: false,
        redirect_url: None,
    }
}

/// Unified entry point for captive portal check (WASM removed).
pub async fn check_for_captive_portal_web(timeout_ms: u64) -> CaptivePortalStatus {
    check_for_captive_portal_web_manual(timeout_ms).await
}
