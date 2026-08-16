//! Main configuration structure

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::providers::ProviderConfig;
use crate::sandbox::SandboxPolicy;
use crate::context::ContextConfig;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Provider configurations
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,

    /// Default provider to use
    #[serde(default = "default_provider")]
    pub default_provider: String,

    /// Default model to use
    #[serde(default)]
    pub default_model: Option<String>,

    /// Sandbox policy
    #[serde(default)]
    pub sandbox: SandboxPolicy,

    /// Context configuration
    #[serde(default)]
    pub context: ContextConfig,

    /// UI/Theme settings
    #[serde(default)]
    pub ui: UiConfig,

    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,

    /// Plugin configurations
    #[serde(default)]
    pub plugins: HashMap<String, toml::Value>,

    /// Custom tool configurations
    #[serde(default)]
    pub tools: HashMap<String, toml::Value>,

    /// Project-specific settings path
    #[serde(skip)]
    pub project_config_path: Option<PathBuf>,

    /// User config path
    #[serde(skip)]
    pub user_config_path: Option<PathBuf>,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Color theme
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Enable colors in output
    #[serde(default = "default_true")]
    pub color: bool,

    /// Show progress indicators
    #[serde(default = "default_true")]
    pub show_progress: bool,

    /// Show tool execution details
    #[serde(default)]
    pub verbose_tools: bool,

    /// Maximum width for text wrapping
    #[serde(default = "default_max_width")]
    pub max_width: usize,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            color: true,
            show_progress: true,
            verbose_tools: false,
            max_width: default_max_width(),
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Log to file
    #[serde(default)]
    pub file: Option<PathBuf>,

    /// Log format (json, pretty)
    #[serde(default = "default_log_format")]
    pub format: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            file: None,
            format: default_log_format(),
        }
    }
}

fn default_provider() -> String {
    "openai".to_string()
}

fn default_theme() -> String {
    "dark".to_string()
}

fn default_true() -> bool {
    true
}

fn default_max_width() -> usize {
    100
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "pretty".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            providers: HashMap::new(),
            default_provider: default_provider(),
            default_model: None,
            sandbox: SandboxPolicy::default(),
            context: ContextConfig::default(),
            ui: UiConfig::default(),
            logging: LoggingConfig::default(),
            plugins: HashMap::new(),
            tools: HashMap::new(),
            project_config_path: None,
            user_config_path: None,
        }
    }
}

impl Config {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from a file
    pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get provider configuration by name
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.providers.get(name)
    }

    /// Merge another configuration into this one
    pub fn merge(&mut self, other: Config) {
        for (key, value) in other.providers {
            self.providers.entry(key).or_insert(value);
        }
        
        if self.default_model.is_none() {
            self.default_model = other.default_model;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_provider, "openai");
        assert!(config.ui.color);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        assert!(!serialized.is_empty());
        
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.default_provider, config.default_provider);
    }
}
