//! The public API of the network_reachability library.

/// Network analysis and statistical tools.
pub mod analysis;
/// Internal constants used by the engine.
pub mod constants;
/// The core network reachability engine and check logic.
pub mod engine;
/// Data structures for configuration, reports, and status.
pub mod models;
/// Individual network probes for DNS, captive portals, etc.
pub mod probes;

// --- Public API Re-exports ---

// Key functions
pub use engine::check_network;

// Core data structures
pub use models::{
    CheckStrategy, ConnectionQuality, NetworkConfiguration, NetworkReport, NetworkStatus,
    NetworkTarget, QualityThresholds, ResilienceConfig, SecurityConfig, TargetProtocol,
};

// Optional, for advanced use
pub use probes::check_for_captive_portal;
