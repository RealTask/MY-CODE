//! File checkpoints and undo functionality.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// A snapshot of file contents that can be restored later.
#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub id: String,
    pub description: String,
    files: HashMap<PathBuf, String>,
}

impl Checkpoint {
    /// Capture the current contents of `paths`.
    pub fn capture<P: AsRef<Path>>(description: impl Into<String>, paths: &[P]) -> Result<Self> {
        let mut files = HashMap::new();
        for path in paths {
            let path = path.as_ref();
            if path.is_file() {
                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("Failed to snapshot {}", path.display()))?;
                files.insert(path.to_path_buf(), content);
            }
        }
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            description: description.into(),
            files,
        })
    }

    /// Restore all files in this checkpoint.
    pub fn restore(&self) -> Result<()> {
        for (path, content) in &self.files {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, content)
                .with_context(|| format!("Failed to restore {}", path.display()))?;
        }
        Ok(())
    }

    /// Number of files captured.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

/// In-memory stack of checkpoints for undo.
#[derive(Debug, Default)]
pub struct CheckpointStore {
    stack: Vec<Checkpoint>,
}

impl CheckpointStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, checkpoint: Checkpoint) {
        self.stack.push(checkpoint);
    }

    /// Undo the most recent checkpoint. Returns the restored checkpoint.
    pub fn undo(&mut self) -> Result<Option<Checkpoint>> {
        if let Some(checkpoint) = self.stack.pop() {
            checkpoint.restore()?;
            Ok(Some(checkpoint))
        } else {
            Ok(None)
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn capture_and_restore() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("a.txt");
        std::fs::write(&file, "one").unwrap();
        let cp = Checkpoint::capture("test", &[&file]).unwrap();
        std::fs::write(&file, "two").unwrap();
        cp.restore().unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "one");
    }
}
