//! Plugin system for custom tools and providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A loaded plugin description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

impl Plugin {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            enabled: true,
        }
    }
}

/// Registry of available plugins.
#[derive(Debug, Default)]
pub struct PluginRegistry {
    plugins: HashMap<String, Plugin>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, plugin: Plugin) {
        self.plugins.insert(plugin.name.clone(), plugin);
    }

    pub fn get(&self, name: &str) -> Option<&Plugin> {
        self.plugins.get(name)
    }

    pub fn list(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_lookup() {
        let mut registry = PluginRegistry::new();
        registry.register(Plugin::new("demo", "0.1.0"));
        assert!(registry.get("demo").unwrap().enabled);
    }
}
