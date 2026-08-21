//! Patch generation and application for safe code modifications.

use anyhow::{Context, Result};
use std::path::Path;

/// Apply a full-file replacement if the current contents still match `original`.
pub fn apply_replacement(path: &Path, original: &str, replacement: &str) -> Result<()> {
    let current = if path.exists() {
        std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {} for edit", path.display()))?
    } else {
        String::new()
    };

    if current != original {
        anyhow::bail!(
            "File {} has changed since it was read; refusing to overwrite",
            path.display()
        );
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, replacement)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

/// Insert `text` after `line_number` (1-indexed). `0` inserts at the start.
pub fn insert_at_line(path: &Path, line_number: usize, text: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let mut lines: Vec<&str> = content.lines().collect();
    let idx = line_number.min(lines.len());
    lines.insert(idx, text);
    let mut new_content = lines.join("\n");
    if content.ends_with('\n') {
        new_content.push('\n');
    }
    std::fs::write(path, new_content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn refuses_stale_replacement() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("f.txt");
        std::fs::write(&file, "old").unwrap();
        let err = apply_replacement(&file, "other", "new").unwrap_err();
        assert!(err.to_string().contains("has changed"));
    }

    #[test]
    fn applies_matching_replacement() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("f.txt");
        std::fs::write(&file, "old").unwrap();
        apply_replacement(&file, "old", "new").unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "new");
    }
}
