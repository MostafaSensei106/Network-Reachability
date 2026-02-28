//! Probe for detecting captive portals.

use crate::api::{constants::LibConstants, models::CaptivePortalStatus};

/// Checks for the presence of a captive portal.
pub async fn check_for_captive_portal(timeout_ms: u64) -> CaptivePortalStatus {
    #[cfg(not(target_arch = "wasm32"))]
    {
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
    {
        check_for_captive_portal_web(timeout_ms).await
    }
}

/// Web implementation using Fetch with redirect: manual.
pub async fn check_for_captive_portal_web_manual(_timeout_ms: u64) -> CaptivePortalStatus {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, RequestRedirect, ResponseType, Window};
        let url = LibConstants::CAPTIVE_PORTAL_DETECTION_URL;
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::NoCors);
        opts.set_redirect(RequestRedirect::Manual);

        let window: Window = web_sys::window().expect("Window not found");
        let request = match Request::new_with_str_and_init(url, &opts) {
            Ok(req) => req,
            Err(_) => {
                return CaptivePortalStatus {
                    is_captive_portal: false,
                    redirect_url: None,
                }
            }
        };

        let fetch_promise = window.fetch_with_request(&request);
        let result = JsFuture::from(fetch_promise).await;

        match result {
            Ok(resp) => {
                let response: web_sys::Response = resp.unchecked_into();
                // In manual redirect mode, a redirect results in an opaque-redirect response
                let is_captive =
                    response.type_() == ResponseType::Opaqueredirect || response.status() == 302;

                CaptivePortalStatus {
                    is_captive_portal: is_captive,
                    redirect_url: None,
                }
            }
            Err(_) => CaptivePortalStatus {
                is_captive_portal: false,
                redirect_url: None,
            },
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = _timeout_ms;
        CaptivePortalStatus {
            is_captive_portal: false,
            redirect_url: None,
        }
    }
}

/// Unified entry point for captive portal check.
pub async fn check_for_captive_portal_web(timeout_ms: u64) -> CaptivePortalStatus {
    check_for_captive_portal_web_manual(timeout_ms).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    #[cfg(not(target_arch = "wasm32"))]
    async fn test_check_for_captive_portal_basic() {
        let status = check_for_captive_portal(500).await;
        assert!(!status.is_captive_portal || status.redirect_url.is_some());
    }
}
