//! Filesystem tools

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Read a file from the filesystem
pub fn read_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", path.display(), e))
}

/// Write content to a file
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    std::fs::write(path, content)
        .map_err(|e| anyhow::anyhow!("Failed to write file {}: {}", path.display(), e))
}

/// Append content to a file
pub fn append_file(path: &Path, content: &str) -> Result<()> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Delete a file
pub fn delete_file(path: &Path) -> Result<()> {
    std::fs::remove_file(path)
        .map_err(|e| anyhow::anyhow!("Failed to delete file {}: {}", path.display(), e))
}

/// Create a directory
pub fn create_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)
        .map_err(|e| anyhow::anyhow!("Failed to create directory {}: {}", path.display(), e))
}

/// Delete a directory recursively
pub fn delete_dir(path: &Path) -> Result<()> {
    std::fs::remove_dir_all(path)
        .map_err(|e| anyhow::anyhow!("Failed to delete directory {}: {}", path.display(), e))
}

/// Move/rename a file
pub fn move_file(from: &Path, to: &Path) -> Result<()> {
    std::fs::rename(from, to).map_err(|e| {
        anyhow::anyhow!(
            "Failed to move file from {} to {}: {}",
            from.display(),
            to.display(),
            e
        )
    })
}

/// Copy a file
pub fn copy_file(from: &Path, to: &Path) -> Result<u64> {
    std::fs::copy(from, to).map_err(|e| {
        anyhow::anyhow!(
            "Failed to copy file from {} to {}: {}",
            from.display(),
            to.display(),
            e
        )
    })
}

/// Check if a path exists
pub fn exists(path: &Path) -> bool {
    path.exists()
}

/// Check if a path is a directory
pub fn is_dir(path: &Path) -> bool {
    path.is_dir()
}

/// Check if a path is a file
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// Get metadata about a file
pub fn metadata(path: &Path) -> Result<std::fs::Metadata> {
    std::fs::metadata(path)
        .map_err(|e| anyhow::anyhow!("Failed to get metadata for {}: {}", path.display(), e))
}

/// List directory contents
pub fn read_dir(path: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let entries = std::fs::read_dir(path)
        .map_err(|e| anyhow::anyhow!("Failed to read directory {}: {}", path.display(), e))?;
    for entry in entries {
        let entry = entry
            .map_err(|e| anyhow::anyhow!("Failed to read directory {}: {}", path.display(), e))?;
        paths.push(entry.path());
    }
    Ok(paths)
}

/// Filesystem tools collection
#[derive(Debug, Default, Clone, Copy)]
pub struct FileSystemTools;

impl FileSystemTools {
    pub fn new() -> Self {
        Self
    }

    /// Read a file
    pub fn read(&self, path: &Path) -> Result<String> {
        read_file(path)
    }

    /// Write to a file
    pub fn write(&self, path: &Path, content: &str) -> Result<()> {
        write_file(path, content)
    }

    /// Delete a file
    pub fn delete(&self, path: &Path) -> Result<()> {
        delete_file(path)
    }

    /// Create a directory
    pub fn create_dir(&self, path: &Path) -> Result<()> {
        create_dir(path)
    }

    /// List directory contents
    pub fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        read_dir(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_read_write_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        write_file(&file_path, "hello world").unwrap();
        let content = read_file(&file_path).unwrap();

        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_file_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        assert!(!exists(&file_path));
        write_file(&file_path, "test").unwrap();
        assert!(exists(&file_path));
    }

    #[test]
    fn test_read_dir_lists_entries() {
        let dir = tempdir().unwrap();
        write_file(&dir.path().join("a.txt"), "a").unwrap();
        let entries = read_dir(dir.path()).unwrap();
        assert_eq!(entries.len(), 1);
    }
}
