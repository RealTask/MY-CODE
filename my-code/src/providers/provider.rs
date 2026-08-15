//! Provider trait and capabilities

use async_trait::async_trait;
use crate::models::{Message, Response, ToolCall};
use crate::errors::provider::ProviderError;
use futures::stream::BoxStream;

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
