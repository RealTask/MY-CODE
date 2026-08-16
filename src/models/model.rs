//! Model information

use serde::{Deserialize, Serialize};

/// Information about a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model identifier/name
    pub id: String,

    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Model description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Context window size (tokens)
    pub context_window: usize,

    /// Maximum output tokens
    pub max_output_tokens: usize,

    /// Whether the model supports streaming
    #[serde(default)]
    pub supports_streaming: bool,

    /// Whether the model supports tool calling
    #[serde(default)]
    pub supports_tool_calling: bool,

    /// Whether the model supports vision
    #[serde(default)]
    pub supports_vision: bool,

    /// Input modalities (text, image, etc.)
    #[serde(default)]
    pub input_modalities: Vec<String>,

    /// Output modalities
    #[serde(default)]
    pub output_modalities: Vec<String>,

    /// Pricing per 1000 tokens (input)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_input_per_1k: Option<f64>,

    /// Pricing per 1000 tokens (output)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_output_per_1k: Option<f64>,
}

impl ModelInfo {
    /// Create a new model info
    pub fn new(id: impl Into<String>, context_window: usize) -> Self {
        Self {
            id: id.into(),
            name: None,
            description: None,
            context_window,
            max_output_tokens: 4096,
            supports_streaming: true,
            supports_tool_calling: false,
            supports_vision: false,
            input_modalities: vec!["text".to_string()],
            output_modalities: vec!["text".to_string()],
            price_input_per_1k: None,
            price_output_per_1k: None,
        }
    }

    /// Set the model name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set whether the model supports tool calling
    pub fn with_tool_calling(mut self, supports: bool) -> Self {
        self.supports_tool_calling = supports;
        self
    }

    /// Set pricing
    pub fn with_pricing(mut self, input: f64, output: f64) -> Self {
        self.price_input_per_1k = Some(input);
        self.price_output_per_1k = Some(output);
        self
    }

    /// Check if this model is suitable for coding tasks
    pub fn is_suitable_for_coding(&self) -> bool {
        // Generally need large context and tool calling for coding
        self.context_window >= 8000 && self.supports_tool_calling
    }
}

impl std::fmt::Display for ModelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} tokens)",
            self.name.as_deref().unwrap_or(&self.id),
            self.context_window
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_creation() {
        let model = ModelInfo::new("gpt-4", 128000)
            .with_name("GPT-4")
            .with_tool_calling(true)
            .with_pricing(0.03, 0.06);

        assert_eq!(model.id, "gpt-4");
        assert_eq!(model.name, Some("GPT-4".to_string()));
        assert!(model.supports_tool_calling);
        assert!(model.is_suitable_for_coding());
    }
}
