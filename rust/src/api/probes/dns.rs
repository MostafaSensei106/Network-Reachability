use std::net::IpAddr;
use tokio::task;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

/// Detects potential DNS hijacking by comparing system DNS resolution with a trusted DoH resolver.
pub async fn detect_dns_hijacking(domain: &str) -> bool {
    // 1. Resolve using the system's default DNS. This is an async operation.
    let system_ips = match tokio::net::lookup_host(format!("{}:443", domain)).await {
        Ok(addrs) => addrs.map(|a| a.ip()).collect::<Vec<_>>(),
        Err(_) => return false, // If system DNS can't resolve, we can't compare.
    };
    if system_ips.is_empty() {
        return false;
    }

    // 2. Resolve using a trusted DoH resolver (Cloudflare).
    // The `lookup_ip` from trust-dns-resolver is a blocking call, so we move it to a blocking thread.
    let domain_for_doh = domain.to_string();
    let doh_ips_res = task::spawn_blocking(move || {
        // Use ResolverConfig::cloudflare() for standard DNS, not cloudflare_https()
        let config = ResolverConfig::cloudflare();
        let doh_resolver =
            Resolver::new(config, ResolverOpts::default()).expect("Failed to create DoH resolver");
        doh_resolver.lookup_ip(&domain_for_doh)
    })
    .await;

    let doh_ips = match doh_ips_res {
        Ok(Ok(lookup)) => lookup.iter().collect::<Vec<IpAddr>>(),
        _ => return false, // If DoH fails, we can't compare.
    };

    // 3. Compare the results
    let is_subset = system_ips.iter().all(|sys_ip| doh_ips.contains(sys_ip));

    !is_subset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_dns_hijacking_clean() {
        // This test assumes a clean network where system DNS and Cloudflare agree.
        // It might be flaky in a network that already has a DNS proxy.
        let is_hijacked = detect_dns_hijacking("www.google.com").await;
        assert!(
            !is_hijacked,
            "Test failed, potential DNS hijack detected in test environment or network issue."
        );
    }
}
