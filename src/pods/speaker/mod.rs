//! Speaker Pod — chat interface for natural language interaction.
//!
//! The simplest pod type. Receives text, runs a rig LLM loop with a system
//! prompt, and emits responses back to the hive.
//!
//! ## Bundle Layout
//!
//! ```text
//! speaker/
//! ├── mod.rs          # Pod trait impl + rig loop
//! ├── prompts/
//! │   └── system.md   # System prompt loaded at init
//! └── knowledge/      # (optional) Tone/style docs for retrieval
//! ```
//!
//! ## Architecture
//!
//! The speaker pod is a [`Pod`] that the hive spawns via [`PodActor`].
//! The hive sends [`Task`] messages; the [`Handler<Task>`] in `PodActor`
//! delegates to the pod's `handle_task` method.
//!
//! Because LLM calls are async, the [`PodActor`] handler for `Task` is
//! customized to await the LLM call directly rather than calling the sync
//! `Pod::handle_task` trait method.

use crate::pod::{Pod, PodActor, PodType, Task, TaskResult};
use actix::prelude::*;
use rig_core::{
    client::CompletionClient,
    completion::Prompt,
    providers::ollama,
};

/// Speaker pod — conversational agent using local Ollama.
///
/// Holds its own rig Ollama client and system prompt, loaded from the bundle
/// at initialization time.
pub struct SpeakerPod {
    /// Ollama client connected to local inference server.
    client: Option<ollama::Client>,
    /// System prompt embedded from prompts/system.md at compile time.
    system_prompt: String,
    /// Model identifier for Ollama.
    model: String,
}

impl SpeakerPod {
    pub fn new() -> Self {
        SpeakerPod {
            client: None,
            system_prompt: include_str!("prompts/system.md").to_string(),
            model: "gemma3:latest".to_string(),
        }
    }

    /// Async LLM call using rig.
    async fn chat(
        client: ollama::Client,
        model: String,
        system_prompt: String,
        input: String,
    ) -> anyhow::Result<String> {
        let agent = client
            .agent(&model)
            .preamble(&system_prompt)
            .build();
        let response = agent.prompt(&input).await?;
        Ok(response)
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
        self.client = Some(ollama::Client::new(rig_core::client::Nothing)?);
        Ok(())
    }

    fn handle_task(
        &mut self,
        task: Task,
    ) -> anyhow::Result<TaskResult> {
        // Sync placeholder — the real LLM call happens in the custom
        // Handler<Task> below which has access to async context.
        let response = format!("[Speaker placeholder for: {}]", task.text);
        Ok(match task.channel {
            Some(ch) => TaskResult::reply(response, ch),
            None => TaskResult::text(response),
        })
    }
}

// ---------------------------------------------------------------------------
// Custom Handler for SpeakerPod that calls LLM async
// ---------------------------------------------------------------------------

/// Override the default Task handler for SpeakerPod to call LLM async.
impl Handler<Task> for PodActor<SpeakerPod> {
    type Result = ResponseActFuture<Self, anyhow::Result<TaskResult>>;

    fn handle(
        &mut self,
        msg: Task,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let input = msg.text;
        let channel = msg.channel;
        let client = self.pod.client.clone();
        let model = self.pod.model.clone();
        let system_prompt = self.pod.system_prompt.clone();

        Box::pin(async move {
            let client = match client {
                Some(c) => c,
                None => return Err(anyhow::anyhow!("Ollama client not initialized")),
            };
            let response = SpeakerPod::chat(client, model, system_prompt, input).await?;
            let result = match channel {
                Some(ch) => TaskResult::reply(response, ch),
                None => TaskResult::text(response),
            };
            Ok(result)
        }.into_actor(self))
    }
}
