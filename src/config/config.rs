//! Main configuration structure

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::context::ContextConfig;
use crate::providers::ProviderConfig;
use crate::sandbox::SandboxPolicy;

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

    /// Load configuration from an optional path, falling back to defaults.
    pub fn load(path: Option<&Path>) -> Result<Self> {
        crate::config::loader::ConfigLoader::load(path)
    }

    /// Load configuration from a specific file.
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file {}", path.display()))?;
        let mut config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file {}", path.display()))?;
        config.project_config_path = Some(path.to_path_buf());
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, content)
            .with_context(|| format!("Failed to write config file {}", path.display()))?;
        Ok(())
    }

    /// Get provider configuration by name
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.providers.get(name)
    }

    /// Merge another configuration into this one. `other` wins on conflicts.
    pub fn merge(&mut self, other: Config) {
        for (key, value) in other.providers {
            self.providers.insert(key, value);
        }

        if !other.default_provider.is_empty() {
            self.default_provider = other.default_provider;
        }

        if other.default_model.is_some() {
            self.default_model = other.default_model;
        }

        self.sandbox = other.sandbox;
        self.context = other.context;
        self.ui = other.ui;
        self.logging = other.logging;

        for (key, value) in other.plugins {
            self.plugins.insert(key, value);
        }
        for (key, value) in other.tools {
            self.tools.insert(key, value);
        }

        if other.project_config_path.is_some() {
            self.project_config_path = other.project_config_path;
        }
        if other.user_config_path.is_some() {
            self.user_config_path = other.user_config_path;
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

    #[test]
    fn load_none_returns_default() {
        let config = Config::load(None).unwrap();
        assert_eq!(config.default_provider, "openai");
    }
}
