//! Data models and core types for the Network Reachability Engine.
//!
//! This module acts as the "Standard Library" of data structures for the entire 
//! project. It is organized into several sub-modules that handle everything 
//! from initial configuration to detailed diagnostic reporting.
//!
//! # Module Overview
//!
//! - **[`config`]**: High-level settings for the engine's behavior 
//!   (strategies, intervals, quality thresholds).
//! - **[`target`]**: Definitions for specific network endpoints 
//!   (hosts, ports, protocols).
//! - **[`net_info`]**: Low-level interface metadata 
//!   (connection types, security flags, traceroute).
//! - **[`report`]**: The final consolidated output of a network check cycle.
//! - **[`error`]**: Categorized failure types for diagnostics.
//!
//! # Common Workflow
//!
//! 1. Create a [`NetworkConfiguration`](config::NetworkConfiguration).
//! 2. Define one or more [`NetworkTarget`](target::NetworkTarget)s.
//! 3. Pass the configuration to the engine.
//! 4. Receive [`NetworkReport`](report::NetworkReport)s describing the 
//!    real-time health and security of the connection.

/// Configuration-related data structures (strategies, thresholds, etc).
pub mod config;
/// Error-related data structures and categories.
pub mod error;
/// Network interface metadata and security status flags.
pub mod net_info;
/// Consolidated check results and statistical metrics.
pub mod report;
/// Definitions for network endpoints and probe protocols.
pub mod target;

// Re-export all sub-module members for easy access via `crate::api::models::*`
pub use config::*;
pub use error::*;
pub use net_info::*;
pub use report::*;
pub use target::*;
