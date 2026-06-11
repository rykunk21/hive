//! Speaker Pod — chat interface for natural language interaction.
//!
//! The simplest pod type. Receives text, runs a rig LLM loop with a system
//! prompt, and emits responses back to the hive.
//!
//! ## Bundle Layout
//!
//! ```
//! speaker/
//! ├── mod.rs          # Pod trait impl + rig loop
//! ├── prompts/
//! │   └── system.md   # System prompt loaded at init
//! └── knowledge/      # (optional) Tone/style docs for retrieval
//! ```
//!
//! ## Rig Loop
//!
//! 1. Receive [`Task`] with text payload
//! 2. Build prompt: system prompt + user message
//! 3. Call LLM via rig
//! 4. Send [`TextResponse`] back to hive via [`MessageRouter`]

use crate::pod::{Pod, PodType, Task, TaskResult};

/// Speaker pod — conversational agent.
///
/// Holds its own rig client and system prompt, loaded from the bundle
/// at initialization time.
pub struct SpeakerPod {
    // TODO: rig_client: rig::Client — LLM client for completions
    system_prompt: String,
}

impl SpeakerPod {
    /// Create a new speaker pod with default configuration.
    pub fn new() -> Self {
        SpeakerPod {
            // TODO: Initialize rig client with config (api_key, model, etc.)
            system_prompt: include_str!("prompts/system.md").to_string(),
        }
    }

    /// Run the rig loop for a single chat turn (blocking, no async yet).
    ///
    /// Builds the full prompt and returns it. Actual LLM call pending rig integration.
    fn chat(&self, input: &str) -> String {
        format!("[{}] User: {}\nAssistant: ", self.system_prompt, input)
    }
}

impl Default for SpeakerPod {
    fn default() -> Self {
        Self::new()
    }
}

impl Pod for SpeakerPod {
    fn pod_type(&self) -> PodType {
        PodType {
            name: "speaker".into(),
        }
    }

    fn init(&mut self) -> anyhow::Result<()> {
        // System prompt already loaded from bundle via include_str!
        // TODO: Validate rig client connectivity, warm caches.
        Ok(())
    }

    fn handle_task(
        &mut self,
        task: Task,
    ) -> anyhow::Result<TaskResult> {
        let response = self.chat(&task.text);
        // Route reply back to the same channel if one was provided
        let result = match task.channel {
            Some(channel) => TaskResult::reply(response, channel),
            None => TaskResult::text(response),
        };
        Ok(result)
    }
}
