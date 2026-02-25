//! Data structures for network configuration, reports, and errors.

/// Configuration-related data structures.
pub mod config;
/// Error-related data structures and types.
pub mod error;
/// Network information and status flags.
pub mod net_info;
/// Comprehensive network reports and statistical metrics.
pub mod report;
/// Definitions for network targets and protocols.
pub mod target;

pub use config::*;
pub use error::*;
pub use net_info::*;
pub use report::*;
pub use target::*;
