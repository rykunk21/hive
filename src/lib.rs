//! Hive library — pod building and runtime support.
//!
//! This crate exposes the core types and traits needed to create pods that
//! participate in a Hive runtime. External pod authors depend on this library
//! to implement the [`Pod`] trait and register their types with the hive.

pub mod config;
pub mod pod;

// TODO: Re-export core pod types for external use once trait stabilizes.
// pub use pod::{Pod, PodType, PodConfig};
