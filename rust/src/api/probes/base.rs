use crate::api::models::{NetworkTarget, TargetReport};
use flutter_rust_bridge::frb;

/// A trait for performing reachability probes across different platforms.
#[frb(ignore)]
pub(crate) trait NetworkProbe {
    /// Performs a reachability check against a specific target.
    fn check(&self, target: &NetworkTarget) -> impl std::future::Future<Output = TargetReport> + Send;
}
