//! Built-in pod types.
//!
//! This module re-exports all pod implementations included in the hive crate.
//! External consumers can depend on this crate and use these pod types in their
//! own Hive runtime, or implement the [`Pod`] trait to create custom types.
//!
//! # Available Pod Types
//!
//! - [`speaker::SpeakerPod`] — audio output and voice synthesis
//!
//! # Example
//!
//! ```rust,ignore
//! use hive::pods::SpeakerPod;
//! use hive::pod::{Pod, PodRegistry};
//!
//! let mut registry = PodRegistry::new();
//! registry.register("speaker", Box::new(|| Box::new(SpeakerPod::default())));
//! ```

pub mod speaker;
