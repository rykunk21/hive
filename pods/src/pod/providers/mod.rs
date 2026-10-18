//! LLM providers for pods.
//!
//! Integrates with `rig` to let pods make LLM calls. During setup, a pod can:
//! - Attach available **local tools**.
//! - Register with **MCP servers** exposed over the API.
//!
//!

use rig_core::{agent::Agent, client::CompletionClient, completion::Prompt, providers::ollama};

#[derive(Clone)]
pub struct Provider {
    client: ollama::Client,
    model: String,
}

impl Provider {
    pub fn new() -> Self {
        let client = ollama::Client::new(rig_core::client::Nothing).expect("failed to init client");
        Self {
            client,
            model: "kimi-k2.6:cloud".into(),
        }
    }

    pub async fn chat(&self, input: String) -> anyhow::Result<String> {
        let agent = self.client.agent(&self.model).build();
        let response = agent.prompt(&input).await?;
        Ok(response)
    }
}

impl Default for Provider {
    fn default() -> Self {
        Self::new()
    }
}

/// Async LLM call using rig
async fn chat_new(client: ollama::Client, model: String, input: String) -> anyhow::Result<String> {
    let agent = client.agent(&model).build();

    let response = agent.prompt(&input).await?;

    Ok(response)
}
