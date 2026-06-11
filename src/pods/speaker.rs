//! Speaker Pod — audio output for voice and notification delivery.
//!
//! This pod type receives text or audio directives and produces spoken output
//! or audible alerts. It runs as an actix actor within a Hive runtime.
//!
//! # Usage
//!
//! ```rust,ignore
//! use hive::pods::speaker::SpeakerPod;
//! use hive::pod::{Pod, PodRegistry};
//!
//! let mut registry = PodRegistry::new();
//! registry.register("speaker", Box::new(|| Box::new(SpeakerPod::default())));
//! ```

use crate::pod::{Pod, PodType, Task, TaskResult};

/// Audio output pod that speaks text or plays notification sounds.
///
/// # Pod Type
/// - Name: `"speaker"`
/// - Domain: audio delivery, voice synthesis, notifications
///
/// # Task Handling
/// Accepts tasks carrying text payloads and routes them to the configured
/// TTS backend or audio player.
pub struct SpeakerPod;

impl Default for SpeakerPod {
    fn default() -> Self {
        SpeakerPod
    }
}

impl Pod for SpeakerPod {
    fn pod_type(&self) -> PodType {
        PodType {
            name: "speaker".into(),
        }
    }

    fn handle_task(&mut self, _task: Task) -> anyhow::Result<TaskResult> {
        todo!("receive text/audio payload, route to TTS or player")
    }
}
