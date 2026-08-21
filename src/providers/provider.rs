//! Provider trait and capabilities

use async_trait::async_trait;
use futures::stream::{self, BoxStream};
use futures::StreamExt;

use crate::errors::provider::ProviderError;
use crate::models::{Message, Response, ToolCall};

/// Capabilities of a model provider
#[derive(Debug, Clone)]
pub struct ProviderCapabilities {
    /// Supports streaming responses
    pub streaming: bool,

    /// Supports tool/function calling
    pub tool_calling: bool,

    /// Supports vision/image input
    pub vision: bool,

    /// Maximum context window size (tokens)
    pub max_context_tokens: usize,

    /// Maximum output tokens
    pub max_output_tokens: usize,

    /// Supported model names
    pub models: Vec<String>,
}

impl Default for ProviderCapabilities {
    fn default() -> Self {
        Self {
            streaming: true,
            tool_calling: true,
            vision: false,
            max_context_tokens: 128_000,
            max_output_tokens: 4_096,
            models: Vec::new(),
        }
    }
}

/// Trait for LLM providers
#[async_trait]
pub trait Provider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;

    /// Get provider capabilities
    fn capabilities(&self) -> &ProviderCapabilities;

    /// Send a chat request and get a response
    async fn chat(
        &self,
        messages: &[Message],
        model: &str,
        system_prompt: Option<&str>,
        tools: Option<&[ToolCall]>,
        temperature: Option<f32>,
        max_tokens: Option<usize>,
    ) -> Result<Response, ProviderError>;

    /// Send a chat request with streaming response
    async fn stream(
        &self,
        messages: &[Message],
        model: &str,
        system_prompt: Option<&str>,
        tools: Option<&[ToolCall]>,
        temperature: Option<f32>,
        max_tokens: Option<usize>,
    ) -> Result<BoxStream<'static, Result<String, ProviderError>>, ProviderError>;

    /// List available models
    async fn list_models(&self) -> Result<Vec<String>, ProviderError>;

    /// Check if the provider is healthy/configured
    async fn health_check(&self) -> Result<bool, ProviderError>;
}

/// A provider that returns canned responses. Useful for tests and dry-runs.
#[derive(Debug, Clone)]
pub struct NullProvider {
    name: String,
    capabilities: ProviderCapabilities,
    reply: String,
}

impl NullProvider {
    pub fn new() -> Self {
        Self {
            name: "null".to_string(),
            capabilities: ProviderCapabilities {
                models: vec!["null".to_string()],
                ..ProviderCapabilities::default()
            },
            reply: "Null provider response".to_string(),
        }
    }

    pub fn with_reply(mut self, reply: impl Into<String>) -> Self {
        self.reply = reply.into();
        self
    }
}

impl Default for NullProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Provider for NullProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn capabilities(&self) -> &ProviderCapabilities {
        &self.capabilities
    }

    async fn chat(
        &self,
        _messages: &[Message],
        model: &str,
        _system_prompt: Option<&str>,
        _tools: Option<&[ToolCall]>,
        _temperature: Option<f32>,
        _max_tokens: Option<usize>,
    ) -> Result<Response, ProviderError> {
        Ok(Response::text(self.reply.clone(), model))
    }

    async fn stream(
        &self,
        _messages: &[Message],
        _model: &str,
        _system_prompt: Option<&str>,
        _tools: Option<&[ToolCall]>,
        _temperature: Option<f32>,
        _max_tokens: Option<usize>,
    ) -> Result<BoxStream<'static, Result<String, ProviderError>>, ProviderError> {
        let reply = self.reply.clone();
        Ok(stream::once(async move { Ok(reply) }).boxed())
    }

    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        Ok(self.capabilities.models.clone())
    }

    async fn health_check(&self) -> Result<bool, ProviderError> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn null_provider_chat() {
        let provider = NullProvider::new().with_reply("hi");
        let response = provider
            .chat(&[Message::user("hello")], "null", None, None, None, None)
            .await
            .unwrap();
        assert_eq!(response.content, "hi");
    }
}
