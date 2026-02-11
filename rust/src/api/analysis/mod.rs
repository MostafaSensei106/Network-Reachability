//! Functions for analyzing raw data from probes to produce insights.

pub mod quality;

pub use quality::{calculate_jitter_stats, evaluate_quality};
