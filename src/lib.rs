//! Hive library — pod building and runtime support.
//!
//! This crate exposes the core types and traits needed to create pods that
//! participate in a Hive runtime. External pod authors depend on this library
//! to implement the [`Pod`] trait and register their types with the hive.
pub mod core;
pub mod knowledge;
pub mod link;
pub mod pods;
pub mod tui;
