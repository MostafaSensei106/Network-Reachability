//! High-level security check orchestration.

use crate::api::{
    models::{NetworkConfiguration, SecurityFlagsResult},
    probes,
};

/// Runs the DNS hijack check by comparing resolution against trusted resolvers.
pub async fn perform_dns_security_check(
    config: &NetworkConfiguration,
    flags: &mut SecurityFlagsResult,
) {
    if !config.security.detect_dns_hijack {
        return;
    }

    let target_to_check = config
        .targets
        .iter()
        .find(|t| t.is_essential)
        .or_else(|| config.targets.first());

    if let Some(target) = target_to_check {
        if probes::detect_dns_hijacking(&target.host).await {
            flags.is_dns_spoofed = true;
        }
    }
}
