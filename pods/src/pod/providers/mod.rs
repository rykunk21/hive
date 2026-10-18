//! LLM providers for pods.
//!
//! Integrates with `rig` to let pods make LLM calls. During setup, a pod can:
//! - Attach available **local tools**.
//! - Register with **MCP servers** exposed over the API.
//!
//!

use std::sync::Arc;

use rig_core::{
    agent::Agent, client::CompletionClient, completion::Prompt, memory::InMemoryConversationMemory,
    providers::ollama,
};

/// Cloneable wrapper around a built `rig` agent so memory/conversation state
/// persists across `.chat()` calls.
#[derive(Clone)]
pub struct Provider {
    agent: Arc<Agent<ollama::CompletionModel>>,
}

impl Provider {
    pub fn new() -> Self {
        let client = ollama::Client::new(rig_core::client::Nothing).expect("failed to init client");
        let agent = Arc::new(
            client
                .agent("kimi-k2.6:cloud")
                .memory(InMemoryConversationMemory::new())
                .build(),
        );

        Self { agent }
    }

    pub async fn chat(&self, input: String) -> anyhow::Result<String> {
        let response = self.agent.prompt(&input).conversation("default").await?;
        Ok(response)
    }
}

impl Default for Provider {
    fn default() -> Self {
        Self::new()
    }
}
