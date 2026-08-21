//! Named configuration profiles.

use serde::{Deserialize, Serialize};

use super::config::Config;

/// A named overlay on top of the base configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    pub name: String,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
}

impl Profile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            provider: None,
            model: None,
        }
    }

    /// Apply this profile onto a config.
    pub fn apply(&self, config: &mut Config) {
        if let Some(provider) = &self.provider {
            config.default_provider = provider.clone();
        }
        if let Some(model) = &self.model {
            config.default_model = Some(model.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_overrides_provider() {
        let mut config = Config::default();
        let mut profile = Profile::new("work");
        profile.provider = Some("anthropic".into());
        profile.apply(&mut config);
        assert_eq!(config.default_provider, "anthropic");
    }
}
