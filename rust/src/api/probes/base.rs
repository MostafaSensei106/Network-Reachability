use crate::api::models::{NetworkTarget, TargetReport};

/// A trait for performing reachability probes across different platforms.
pub trait NetworkProbe {
    /// Performs a reachability check against a specific target.
    fn check(&self, target: &NetworkTarget) -> impl std::future::Future<Output = TargetReport> + Send;
}
