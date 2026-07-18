//! Built-in pod types.
//!
//! This module re-exports all pod implementations included in the hive crate.
//! External consumers can depend on this crate and use these pod types in their
//! own Hive runtime, or implement the [`Pod`] trait to create custom types.
//!
//! Each pod type is a self-contained bundle:
//! - `mod.rs` — Pod trait impl + rig loop
//! - `prompts/` — System prompts and few-shot examples
//! - `scripts/` — Helper scripts the pod can shell out to
//! - `knowledge/` — Domain docs indexed for retrieval
//!
//! # Available Pod Types
//!
//! - [`speaker::SpeakerPod`] — chat interface via LLM loop
//!
#![allow(dead_code)]
pub mod pod;

use pod::Pod;

pub mod ping;
pub mod speaker;
