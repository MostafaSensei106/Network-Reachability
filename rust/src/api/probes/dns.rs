//! Probe for detecting DNS hijacking.

/// Detects potential DNS hijacking.
#[cfg(not(target_arch = "wasm32"))]
pub async fn detect_dns_hijacking(domain: &str) -> bool {
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
        let doh_resolver = match Resolver::new(config, ResolverOpts::default()) {
            Ok(r) => r,
            Err(_) => return Err(()),
        };
        Ok(doh_resolver.lookup_ip(&domain_for_doh))
    })
    .await;

    let doh_ips = match doh_ips_res {
        Ok(Ok(Ok(lookup))) => lookup.iter().collect::<Vec<IpAddr>>(),
        _ => return false,
    };

    // 3. Compare the results
    let is_subset = system_ips.iter().all(|sys_ip| doh_ips.contains(sys_ip));

    !is_subset
}

#[cfg(target_arch = "wasm32")]
pub async fn detect_dns_hijacking(_domain: &str) -> bool {
    false
}
