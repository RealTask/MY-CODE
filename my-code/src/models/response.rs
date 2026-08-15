//! Response types

use serde::{Deserialize, Serialize};
use crate::models::{ToolCall, Usage};

/// Response from a model provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Unique identifier for the response
    pub id: String,

    /// The model that generated this response
    pub model: String,

    /// Content of the response (text)
    #[serde(default)]
    pub content: String,

    /// Tool calls in the response (if any)
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,

    /// Token usage information
    #[serde(default)]
    pub usage: Usage,

    /// Finish reason (stop, length, tool_calls, etc.)
    #[serde(default)]
    pub finish_reason: Option<String>,

    /// Whether this is a streaming chunk
    #[serde(default)]
    pub is_streaming: bool,
}

impl Response {
    /// Create a new response with just content
    pub fn text(content: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            model: model.into(),
            content: content.into(),
            tool_calls: Vec::new(),
            usage: Usage::default(),
            finish_reason: Some("stop".to_string()),
            is_streaming: false,
        }
    }

    /// Create a response with tool calls
    pub fn with_tool_calls(
        tool_calls: Vec<ToolCall>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            model: model.into(),
            content: String::new(),
            tool_calls,
            usage: Usage::default(),
            finish_reason: Some("tool_calls".to_string()),
            is_streaming: false,
        }
    }

    /// Check if this response contains tool calls
    pub fn has_tool_calls(&self) -> bool {
        !self.tool_calls.is_empty()
    }

    /// Check if this is a text-only response
    pub fn is_text_only(&self) -> bool {
        self.tool_calls.is_empty() && !self.content.is_empty()
    }

    /// Get the content or an empty string
    pub fn content_str(&self) -> &str {
        &self.content
    }
}

impl From<String> for Response {
    fn from(content: String) -> Self {
        Response::text(content, "unknown")
    }
}

impl From<&str> for Response {
    fn from(content: &str) -> Self {
        Response::text(content, "unknown")
    }
}
