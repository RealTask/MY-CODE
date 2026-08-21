//! Configuration loading from files and environment.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::config::Config;

/// Loads configuration from well-known locations.
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration, preferring `explicit` then project then user config.
    pub fn load(explicit: Option<&Path>) -> Result<Config> {
        if let Some(path) = explicit {
            return Config::load_from_file(path);
        }

        for candidate in Self::search_paths() {
            if candidate.is_file() {
                return Config::load_from_file(&candidate);
            }
        }

        Ok(Config::default())
    }

    /// Paths checked when no explicit config is given, in priority order.
    pub fn search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        if let Ok(cwd) = std::env::current_dir() {
            paths.push(cwd.join("my-code.toml"));
            paths.push(cwd.join(".my-code.toml"));
            paths.push(cwd.join(".my-code").join("config.toml"));
        }
        if let Some(dir) = crate::utils::paths::Paths::config_dir() {
            paths.push(dir.join("config.toml"));
        }
        paths
    }

    /// Load and merge user + project configs.
    pub fn load_layered() -> Result<Config> {
        let mut config = Config::default();
        if let Some(dir) = crate::utils::paths::Paths::config_dir() {
            let user = dir.join("config.toml");
            if user.is_file() {
                let user_cfg = Config::load_from_file(&user)
                    .with_context(|| format!("Failed to load {}", user.display()))?;
                config.merge(user_cfg);
            }
        }
        if let Ok(cwd) = std::env::current_dir() {
            let project = cwd.join("my-code.toml");
            if project.is_file() {
                let project_cfg = Config::load_from_file(&project)?;
                config.merge(project_cfg);
            }
        }
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn missing_explicit_file_errors() {
        let err = ConfigLoader::load(Some(Path::new("/no/such/my-code-config.toml"))).unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn loads_from_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(&path, "default_provider = \"anthropic\"\n").unwrap();
        let config = ConfigLoader::load(Some(&path)).unwrap();
        assert_eq!(config.default_provider, "anthropic");
    }
}
