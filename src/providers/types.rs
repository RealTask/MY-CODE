//! Provider configuration types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for a model provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider type (openai, anthropic, google, etc.)
    #[serde(rename = "type")]
    pub provider_type: String,

    /// API key (can be omitted if using environment variable)
    pub api_key: Option<String>,

    /// API endpoint URL (for self-hosted or compatible APIs)
    pub base_url: Option<String>,

    /// Default model to use with this provider
    pub default_model: Option<String>,

    /// Available models
    #[serde(default)]
    pub models: Vec<String>,

    /// Request timeout in seconds
    pub timeout_secs: Option<u64>,

    /// Maximum retries for failed requests
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Additional headers
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// Custom parameters for the provider
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

fn default_max_retries() -> u32 {
    3
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider_type: "openai".to_string(),
            api_key: None,
            base_url: None,
            default_model: None,
            models: Vec::new(),
            timeout_secs: Some(60),
            max_retries: default_max_retries(),
            headers: HashMap::new(),
            extra: HashMap::new(),
        }
    }
}

impl ProviderConfig {
    /// Create a new provider config with just the type
    pub fn new(provider_type: &str) -> Self {
        Self {
            provider_type: provider_type.to_string(),
            ..Default::default()
        }
    }

    /// Get API key from config or environment
    pub fn get_api_key(&self, env_var: &str) -> Option<String> {
        self.api_key.clone().or_else(|| std::env::var(env_var).ok())
    }

    /// Check if provider is configured
    pub fn is_configured(&self, env_var: &str) -> bool {
        self.get_api_key(env_var).is_some()
    }

    /// Validate configuration
    pub fn validate(&self, env_var: &str) -> Result<(), String> {
        if !self.is_configured(env_var) && self.provider_type != "local" {
            return Err(format!(
                "Provider '{}' requires an API key. Set 'api_key' in config or {} environment variable.",
                self.provider_type, env_var
            ));
        }
        Ok(())
    }
}
