//! Functions for performing individual network checks (probes).

/// The base trait for all network probes.
pub mod base;
/// Probes for detecting captive portals (login pages).
pub mod captive_portal;
/// Probes for DNS integrity and hijacking detection.
pub mod dns;
/// System-level network interface inspection.
pub mod interface;
/// Probes for individual target reachability.
pub mod target;

// Re-export public functions for easy access from the engine
pub use captive_portal::{check_for_captive_portal, check_for_captive_portal_web};
pub use dns::{detect_dns_hijacking, detect_dns_hijacking_web};
pub use interface::{detect_security_and_network_type, detect_security_and_network_type_web};
pub use target::check_target;
// pub use traceroute::trace_route;
