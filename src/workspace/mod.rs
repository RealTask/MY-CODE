//! Project detection, scanning, and workspace understanding.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// A project workspace rooted at a directory.
#[derive(Debug, Clone)]
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    /// Create a workspace for an existing directory.
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        if !root.exists() {
            anyhow::bail!("Workspace path does not exist: {}", root.display());
        }
        Ok(Self {
            root: crate::utils::paths::Paths::normalize(&root),
        })
    }

    /// Detect a workspace from the current directory.
    pub fn detect() -> Result<Self> {
        let cwd = std::env::current_dir().context("Failed to determine current directory")?;
        Self::from_path(cwd)
    }

    /// Walk parents looking for a project root (git repo or known manifest).
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut current = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        loop {
            if is_project_root(&current) {
                return Self::new(current);
            }
            if !current.pop() {
                break;
            }
        }

        Self::new(path)
    }

    /// Workspace root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Resolve a path relative to the workspace, rejecting escapes.
    pub fn resolve(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        let path = path.as_ref();
        let joined = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        };
        let normalized = crate::utils::paths::Paths::normalize(&joined);
        if !crate::utils::paths::Paths::is_within(&self.root, &normalized) {
            anyhow::bail!("Path is outside the workspace: {}", path.display());
        }
        Ok(normalized)
    }
}

fn is_project_root(path: &Path) -> bool {
    path.join(".git").exists()
        || path.join("Cargo.toml").exists()
        || path.join("package.json").exists()
        || path.join("pyproject.toml").exists()
        || path.join("go.mod").exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn rejects_missing_directory() {
        let err = Workspace::new("/this/does/not/exist-my-code").unwrap_err();
        assert!(err.to_string().contains("does not exist"));
    }

    #[test]
    fn resolve_stays_inside_workspace() {
        let dir = tempdir().unwrap();
        let ws = Workspace::new(dir.path()).unwrap();
        let resolved = ws.resolve("src/main.rs").unwrap();
        assert!(resolved.starts_with(dir.path()));
        assert!(ws.resolve("../outside").is_err());
    }
}
