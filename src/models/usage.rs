//! Token usage tracking

use serde::{Deserialize, Serialize};

/// Token usage information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usage {
    /// Number of tokens in the prompt/input
    #[serde(default)]
    pub prompt_tokens: usize,

    /// Number of tokens in the completion/output
    #[serde(default)]
    pub completion_tokens: usize,

    /// Total tokens used (prompt + completion)
    #[serde(default)]
    pub total_tokens: usize,

    /// Estimated cost in USD (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_cost_usd: Option<f64>,
}

impl Usage {
    /// Create new usage with specific token counts
    pub fn new(prompt_tokens: usize, completion_tokens: usize) -> Self {
        let total_tokens = prompt_tokens + completion_tokens;
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens,
            estimated_cost_usd: None,
        }
    }

    /// Add another usage to this one
    pub fn add(&mut self, other: &Usage) {
        self.prompt_tokens += other.prompt_tokens;
        self.completion_tokens += other.completion_tokens;
        self.total_tokens += other.total_tokens;
        
        if let Some(cost) = other.estimated_cost_usd {
            self.estimated_cost_usd = 
                Some(self.estimated_cost_usd.unwrap_or(0.0) + cost);
        }
    }

    /// Calculate estimated cost based on model pricing
    pub fn calculate_cost(&mut self, prompt_price: f64, completion_price: f64) {
        // Prices are per 1000 tokens
        let cost = (self.prompt_tokens as f64 * prompt_price / 1000.0)
            + (self.completion_tokens as f64 * completion_price / 1000.0);
        self.estimated_cost_usd = Some(cost);
    }

    /// Check if usage is empty
    pub fn is_empty(&self) -> bool {
        self.total_tokens == 0
    }

    /// Format usage for display
    pub fn format(&self) -> String {
        format!(
            "{} tokens ({} in, {} out)",
            self.total_tokens, self.prompt_tokens, self.completion_tokens
        )
    }
}

impl std::fmt::Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_creation() {
        let usage = Usage::new(100, 50);
        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.completion_tokens, 50);
        assert_eq!(usage.total_tokens, 150);
    }

    #[test]
    fn test_usage_add() {
        let mut usage = Usage::new(100, 50);
        let other = Usage::new(200, 100);
        usage.add(&other);
        
        assert_eq!(usage.prompt_tokens, 300);
        assert_eq!(usage.completion_tokens, 150);
        assert_eq!(usage.total_tokens, 450);
    }

    #[test]
    fn test_usage_format() {
        let usage = Usage::new(100, 50);
        let formatted = usage.format();
        assert!(formatted.contains("150"));
        assert!(formatted.contains("100"));
        assert!(formatted.contains("50"));
    }
}
