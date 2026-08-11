//! Probe for detecting captive portals.

use crate::api::{constants::LibConstants, models::CaptivePortalStatus};

/// Checks for the presence of a captive portal.
#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
pub async fn check_for_captive_portal(_timeout_ms: u64) -> CaptivePortalStatus {
    CaptivePortalStatus {
        is_captive_portal: false,
        redirect_url: None,
    }
}
