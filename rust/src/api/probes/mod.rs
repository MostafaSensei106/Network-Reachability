//! Functions for performing individual network checks (probes).

pub mod captive_portal;
pub mod dns;
pub mod interface;
pub mod local_scan;
pub mod target;
pub mod traceroute;

// Re-export public functions for easy access from the engine
pub use captive_portal::check_for_captive_portal;
pub use dns::detect_dns_hijacking;
pub use interface::detect_security_and_network_type;
pub use local_scan::scan_local_network;
pub use target::check_target;
pub use traceroute::trace_route;
