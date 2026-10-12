use actix::prelude::*;
use rig_core::{client::CompletionClient, completion::Prompt, providers::ollama};

/// Type of message that Speaker can receive
#[derive(Message)]
#[rtype(result = "SpeakerResponse")]
pub enum SpeakerMessage {
    Prompt(String),
}

/// Type of message that Speaker can return
pub enum SpeakerResponse {
    Response(String),
}

/// Actor
pub struct Speaker {
    /// Ollama client connected to local inference server.
    client: Option<ollama::Client>,

    /// System prompt embedded from prompts/system.md at compile time.
    system_prompt: String,

    /// Model identifier for Ollama.
    model: String,
}

impl Speaker {
    pub fn new() -> Self {
        Self {
            client: Some(
                ollama::Client::new(rig_core::client::Nothing).expect("failed to init client"),
            ),
            system_prompt: include_str!("prompts/system.md").to_string(),
            model: "qwen2.5:1.5b".to_string(),
        }
    }

    /// Async generation wrapper
    async fn generate(
        client: ollama::Client,
        model: String,
        system_prompt: String,
        prompt: String,
    ) -> SpeakerResponse {
        match Speaker::chat(client, model, system_prompt, prompt).await {
            Ok(response) => SpeakerResponse::Response(response),
            Err(e) => panic!("Speaker::chat failed: {}", e),
        }
    }

    /// Async LLM call using rig
    async fn chat(
        client: ollama::Client,
        model: String,
        system_prompt: String,
        input: String,
    ) -> anyhow::Result<String> {
        let agent = client.agent(&model).preamble(&system_prompt).build();

        let response = agent.prompt(&input).await?;

        Ok(response)
    }
}

impl Default for Speaker {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for Speaker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {}

    fn stopped(&mut self, _ctx: &mut Self::Context) {}
}

/// Message handler
impl Handler<SpeakerMessage> for Speaker {
    type Result = ResponseFuture<SpeakerResponse>;

    fn handle(&mut self, msg: SpeakerMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let client = self.client.clone();
        let model = self.model.clone();
        let system_prompt = self.system_prompt.clone();

        Box::pin(async move {
            let client = match client {
                Some(c) => c,
                None => {
                    return SpeakerResponse::Response("Ollama client not initialized".to_string());
                }
            };

            match msg {
                SpeakerMessage::Prompt(prompt) => {
                    Speaker::generate(client, model, system_prompt, prompt).await
                }
            }
        })
    }
}
