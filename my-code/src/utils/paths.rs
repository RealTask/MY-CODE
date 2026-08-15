//! Path utilities for handling filesystem operations

use std::path::{Path, PathBuf};
use anyhow::Result;

/// Utility functions for path operations
pub struct Paths;

impl Paths {
    /// Normalize a path, resolving `.` and `..` components
    pub fn normalize(path: &Path) -> PathBuf {
        let mut components = path.components().peekable();
        let mut ret = if let Some(c @ prefix::Prefix(..)) = components.peek().copied() {
            components.next();
            PathBuf::from(c.as_os_str())
        } else {
            PathBuf::new()
        };

        for component in components {
            match component {
                prefix::Prefix(..) => unreachable!(),
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
            }
        }

        ret
    }

    /// Check if a path is within a base directory
    pub fn is_within(base: &Path, path: &Path) -> bool {
        let base = Self::normalize(base);
        let path = Self::normalize(path);
        path.starts_with(&base)
    }

    /// Get the home directory
    pub fn home_dir() -> Option<PathBuf> {
        directories::UserDirs::new().map(|d| d.home_dir().to_path_buf())
    }

    /// Get the config directory for MY CODE
    pub fn config_dir() -> Option<PathBuf> {
        directories::ConfigDir::new().map(|d| d.join("my-code"))
    }

    /// Get the data directory for MY CODE
    pub fn data_dir() -> Option<PathBuf> {
        directories::DataDir::new().map(|d| d.join("my-code"))
    }

    /// Get the cache directory for MY CODE
    pub fn cache_dir() -> Option<PathBuf> {
        directories::CacheDir::new().map(|d| d.join("my-code"))
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

        assert!(Paths::is_within(base, within));
        assert!(!Paths::is_within(base, outside));
    }
}
