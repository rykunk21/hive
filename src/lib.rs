//! Hive library — pod building and runtime support.
//!
//! This crate exposes the core types and traits needed to create pods that
//! participate in a Hive runtime. External pod authors depend on this library
//! to implement the [`Pod`] trait and register their types with the hive.

pub mod config;
pub mod hive;
pub mod knowledge;
pub mod link;
pub mod pod;
pub mod pods;
pub mod tui;

// Re-export core types for external pod authors
pub use pod::{Pod, PodActor, PodRegistry, PodType, Task, TaskResult};
