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
//! ## Rig Loop
//!
//! 1. Receive [`Task`] with text payload
//! 2. Build prompt: system prompt + user message
//! 3. Call LLM via rig Ollama provider
//! 4. Send [`TaskResult`] back to hive

use crate::pod::{Pod, PodType, Task, TaskResult, TaskFuture};
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
    /// Ollama client connected to local inference server (default: localhost:11434).
    /// Initialized in `init()` so connection errors surface at actor startup.
    client: Option<ollama::Client>,
    /// System prompt embedded from prompts/system.md at compile time.
    system_prompt: String,
    /// Model identifier for Ollama (e.g., "qwen2.5:14b", "llama3.2").
    model: String,
}

impl SpeakerPod {
    /// Create a new speaker pod with default configuration.
    ///
    /// The Ollama client is not connected yet — call `init()` before use.
    pub fn new() -> Self {
        SpeakerPod {
            client: None,
            system_prompt: include_str!("prompts/system.md").to_string(),
            model: "gemma3:latest".to_string(),
        }
    }

    /// Run the rig loop for a single chat turn.
    ///
    /// Clones the client handle so the returned future is 'static.
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
        // Connect to local Ollama server (no auth required by default).
        // Fails fast here so the actor stops if Ollama is not running.
        self.client = Some(ollama::Client::new(rig_core::client::Nothing)?);
        Ok(())
    }

    fn handle_task(
        &mut self,
        task: Task,
    ) -> TaskFuture {
        let input = task.text;
        let channel = task.channel;

        // Clone the client so the future is 'static
        let client = match &self.client {
            Some(c) => c.clone(),
            None => {
                return Box::pin(async move {
                    Err(anyhow::anyhow!("Ollama client not initialized — call init() first"))
                });
            }
        };

        let model = self.model.clone();
        let system_prompt = self.system_prompt.clone();

        Box::pin(async move {
            let response = SpeakerPod::chat(client, model, system_prompt, input).await?;

            let result = match channel {
                Some(ch) => TaskResult::reply(response, ch),
                None => TaskResult::text(response),
            };
            Ok(result)
        })
    }
}
