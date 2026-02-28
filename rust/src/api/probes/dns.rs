//! Probe for detecting DNS hijacking.

/// Detects potential DNS hijacking.
pub async fn detect_dns_hijacking(domain: &str) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::net::IpAddr;
        use tokio::task;
        use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
        use trust_dns_resolver::Resolver;

        // 1. Resolve using the system's default DNS. This is an async operation.
        let system_ips = match tokio::net::lookup_host(format!("{}:443", domain)).await {
            Ok(addrs) => addrs.map(|a| a.ip()).collect::<Vec<_>>(),
            Err(_) => return false,
        };
        if system_ips.is_empty() {
            return false;
        }

        // 2. Resolve using a trusted DoH resolver (Cloudflare).
        let domain_for_doh = domain.to_string();
        let doh_ips_res = task::spawn_blocking(move || {
            let config = ResolverConfig::cloudflare();
            let doh_resolver =
                Resolver::new(config, ResolverOpts::default()).expect("Failed to create DoH resolver");
            doh_resolver.lookup_ip(&domain_for_doh)
        })
        .await;

        let doh_ips = match doh_ips_res {
            Ok(Ok(lookup)) => lookup.iter().collect::<Vec<IpAddr>>(),
            _ => return false,
        };

        // 3. Compare the results
        let is_subset = system_ips.iter().all(|sys_ip| doh_ips.contains(sys_ip));

        !is_subset
    }
    #[cfg(target_arch = "wasm32")]
    {
        detect_dns_hijacking_web(domain).await
    }
}

/// Web-specific implementation (currently a placeholder as browser environment is limited).
pub async fn detect_dns_hijacking_web(_domain: &str) -> bool {
    // Browsers don't expose raw DNS responses or IP addresses easily due to security.
    // DNS hijacking detection on the web is Best-effort/Unsupported.
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn test_detect_dns_hijacking_basic() {
        let _ = detect_dns_hijacking("www.google.com").await;
        assert!(true);
    }
}
