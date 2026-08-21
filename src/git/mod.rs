//! Git repository operations and change tracking.

use anyhow::Result;
use std::path::{Path, PathBuf};

/// A git repository rooted at `path`.
#[derive(Debug, Clone)]
pub struct Repository {
    path: PathBuf,
}

impl Repository {
    /// Open a repository if `.git` exists at `path`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !crate::tools::git::is_git_repo(&path) {
            anyhow::bail!("Not a git repository: {}", path.display());
        }
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn status(&self) -> Result<crate::tools::git::GitStatus> {
        crate::tools::git::get_status(&self.path)
    }

    pub fn diff(&self, staged: bool) -> Result<String> {
        crate::tools::git::get_diff(&self.path, staged)
    }
}

/// Discover a git repository starting at `path` and walking parents.
pub fn discover(path: impl AsRef<Path>) -> Result<Repository> {
    let mut current = path.as_ref().to_path_buf();
    loop {
        if crate::tools::git::is_git_repo(&current) {
            return Repository::open(current);
        }
        if !current.pop() {
            break;
        }
    }
    anyhow::bail!("No git repository found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn open_rejects_non_repo() {
        let dir = tempdir().unwrap();
        assert!(Repository::open(dir.path()).is_err());
    }
}
