//! Functions for performing individual network checks (probes).

/// Probes for detecting captive portals (login pages).
pub mod captive_portal;
/// Probes for DNS integrity and hijacking detection.
pub mod dns;
/// System-level network interface inspection.
pub mod interface;
/// Probes for individual target reachability.
pub mod target;
/// Traceroute implementation (currently experimental).
pub mod traceroute;

// Re-export public functions for easy access from the engine
pub use captive_portal::check_for_captive_portal;
pub use dns::detect_dns_hijacking;
pub use interface::detect_security_and_network_type;
pub use target::check_target;
// pub use traceroute::trace_route;
