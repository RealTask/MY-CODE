//! Search tools for files and text

use anyhow::Result;
use std::path::{Path, PathBuf};
use glob::Pattern;
use walkdir::WalkDir;

/// Search result item
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub line_number: Option<usize>,
    pub content: String,
}

/// Search for files by pattern
pub fn find_files(pattern: &str, root: &Path) -> Result<Vec<PathBuf>> {
    let glob_pattern = Pattern::new(pattern)?;
    let mut results = Vec::new();
    
    for entry in WalkDir::new(root).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            if glob_pattern.matches(path_str) {
                results.push(path.to_path_buf());
            }
        }
    }
    
    Ok(results)
}

/// Search for text in files (simple grep-like)
pub fn search_text(
    query: &str,
    root: &Path,
    file_pattern: Option<&str>,
    max_results: usize,
) -> Result<Vec<SearchResult>> {
    let mut results = Vec::new();
    let pattern = file_pattern.map(Pattern::new).transpose()?;
    
    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if results.len() >= max_results {
            break;
        }
        
        let path = entry.path();
        
        // Skip directories and binary files
        if !path.is_file() {
            continue;
        }
        
        // Check file pattern if provided
        if let Some(ref pat) = pattern {
            if let Some(path_str) = path.to_str() {
                if !pat.matches(path_str) {
                    continue;
                }
            }
        }
        
        // Try to read as text
        if let Ok(content) = std::fs::read_to_string(path) {
            for (line_num, line) in content.lines().enumerate() {
                if line.contains(query) {
                    results.push(SearchResult {
                        path: path.to_path_buf(),
                        line_number: Some(line_num + 1),
                        content: line.trim().to_string(),
                    });
                    
                    if results.len() >= max_results {
                        break;
                    }
                }
            }
        }
    }
    
    Ok(results)
}

/// Get file info
pub fn get_file_info(path: &Path) -> Result<FileInfo> {
    let metadata = std::fs::metadata(path)?;
    
    Ok(FileInfo {
        path: path.to_path_buf(),
        size: metadata.len(),
        is_file: metadata.is_file(),
        is_dir: metadata.is_dir(),
        modified: metadata.modified().ok(),
        created: metadata.created().ok(),
    })
}

/// File information
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub is_file: bool,
    pub is_dir: bool,
    pub modified: Option<std::time::SystemTime>,
    pub created: Option<std::time::SystemTime>,
}

/// List files in directory
pub fn list_files(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if recursive {
        for entry in WalkDir::new(dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    } else {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if entry.path().is_file() {
                files.push(entry.path());
            }
        }
    }
    
    Ok(files)
}

/// Search tools collection
pub struct SearchTools;

impl SearchTools {
    /// Find files matching pattern
    pub fn find_files(pattern: &str, root: &Path) -> Result<Vec<PathBuf>> {
        find_files(pattern, root)
    }
    
    /// Search for text in files
    pub fn search_text(query: &str, root: &Path) -> Result<Vec<SearchResult>> {
        search_text(query, root, None, 100)
    }
    
    /// List files in directory
    pub fn list_files(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
        list_files(dir, recursive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_find_files() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("test.rs"), "fn main() {}").unwrap();
        std::fs::write(dir.path().join("other.txt"), "hello").unwrap();
        
        let results = find_files("*.rs", dir.path()).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].ends_with("test.rs"));
    }
    
    #[test]
    fn test_search_text() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("test.txt"), "hello world\ntest line\nhello again").unwrap();
        
        let results = search_text("hello", dir.path(), None, 10).unwrap();
        assert_eq!(results.len(), 2);
    }
}
