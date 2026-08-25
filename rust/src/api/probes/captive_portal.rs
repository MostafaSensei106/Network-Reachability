#[cfg(not(target_arch = "wasm32"))]
use crate::api::constants::LibConstants;
use crate::api::models::CaptivePortalStatus;

/// Checks for the presence of a captive portal.
#[cfg(not(target_arch = "wasm32"))]
pub async fn check_for_captive_portal(timeout_ms: u64) -> CaptivePortalStatus {
    use std::time::Duration;

    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    let client = CLIENT.get_or_init(|| {
        reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::limited(5)) // Follow up to 5 redirects
            .build()
            .unwrap_or_default()
    });

    let url = LibConstants::CAPTIVE_PORTAL_DETECTION_URL;

    match client
        .get(url)
        .timeout(Duration::from_millis(timeout_ms))
        .send()
        .await
    {
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
