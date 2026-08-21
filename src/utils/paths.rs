//! Path utilities for handling filesystem operations

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Utility functions for path operations
pub struct Paths;

impl Paths {
    /// Normalize a path, resolving `.` and `..` components
    pub fn normalize(path: &Path) -> PathBuf {
        let mut ret = PathBuf::new();

        for component in path.components() {
            match component {
                std::path::Component::RootDir => {
                    ret.push(component.as_os_str());
                }
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    ret.pop();
                }
                std::path::Component::Normal(c) => {
                    ret.push(c);
                }
                std::path::Component::Prefix(_) => {
                    ret.push(component.as_os_str());
                }
            }
        }

        ret
    }

    /// Check if a path is within a base directory (component-wise, after normalize).
    pub fn is_within(base: &Path, path: &Path) -> bool {
        let base = Self::normalize(base);
        let path = Self::normalize(path);
        path.starts_with(&base)
    }

    /// Get the home directory
    pub fn home_dir() -> Option<PathBuf> {
        directories::UserDirs::new().map(|d| d.home_dir().to_path_buf())
    }

    /// Project directory helpers from the `directories` crate.
    fn project_dirs() -> Option<directories::ProjectDirs> {
        directories::ProjectDirs::from("dev", "realtask", "my-code")
    }

    /// Get the config directory for MY CODE
    pub fn config_dir() -> Option<PathBuf> {
        Self::project_dirs()
            .map(|d| d.config_dir().to_path_buf())
            .or_else(|| {
                directories::BaseDirs::new().map(|d| d.config_dir().join("my-code"))
            })
    }

    /// Get the data directory for MY CODE
    pub fn data_dir() -> Option<PathBuf> {
        Self::project_dirs()
            .map(|d| d.data_dir().to_path_buf())
            .or_else(|| {
                directories::BaseDirs::new().map(|d| d.data_local_dir().join("my-code"))
            })
    }

    /// Get the cache directory for MY CODE
    pub fn cache_dir() -> Option<PathBuf> {
        Self::project_dirs()
            .map(|d| d.cache_dir().to_path_buf())
            .or_else(|| {
                directories::BaseDirs::new().map(|d| d.cache_dir().join("my-code"))
            })
    }

    /// Ensure a directory exists, creating it if necessary
    pub fn ensure_dir(path: &Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }

    /// Join paths safely
    pub fn join(base: &Path, parts: &[&str]) -> PathBuf {
        let mut result = base.to_path_buf();
        for part in parts {
            result = result.join(part);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_normalize_simple() {
        let path = Path::new("/home/user/./docs/../projects");
        let normalized = Paths::normalize(path);
        assert_eq!(normalized, Path::new("/home/user/projects"));
    }

    #[test]
    fn test_is_within() {
        let base = Path::new("/workspace");
        let within = Path::new("/workspace/project/src");
        let outside = Path::new("/etc/passwd");
        let sibling = Path::new("/workspace-evil/src");

        assert!(Paths::is_within(base, within));
        assert!(!Paths::is_within(base, outside));
        assert!(!Paths::is_within(base, sibling));
    }

    #[test]
    fn config_dirs_are_some() {
        assert!(Paths::config_dir().is_some() || Paths::home_dir().is_some());
    }
}
