//! Functions for analyzing raw data from probes to produce insights.

pub mod quality;
pub mod stats;

pub use quality::{evaluate_network_quality, evaluate_quality};
pub use stats::{calculate_jitter_stats, compute_latency_stats};
