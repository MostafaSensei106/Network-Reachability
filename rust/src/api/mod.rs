//! The public API of the network_reachability library.

// Feature-based organization
pub mod analysis;
pub mod constants;
pub mod engine;
pub mod models;
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
pub use probes::{check_for_captive_portal, scan_local_network, trace_route};
