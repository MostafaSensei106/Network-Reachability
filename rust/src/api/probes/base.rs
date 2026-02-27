use crate::api::models::{NetworkTarget, TargetReport};
use async_trait::async_trait;

/// A trait for performing reachability probes across different platforms.
/// We use ?Send only on WASM because WASM futures (JsFuture) are not Send.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait NetworkProbe {
    /// Performs a reachability check against a specific target.
    async fn check(&self, target: &NetworkTarget) -> TargetReport;
}
