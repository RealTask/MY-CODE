//! Code indexing and symbol tracking.

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::tools::code::{analyze_symbols, Symbol};

/// An in-memory index of symbols in a set of files.
#[derive(Debug, Default)]
pub struct CodeIndex {
    symbols: Vec<Symbol>,
    files: Vec<PathBuf>,
}

impl CodeIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Index a single source file.
    pub fn index_file(&mut self, path: &Path) -> Result<()> {
        let symbols = analyze_symbols(path)?;
        self.symbols.extend(symbols);
        self.files.push(path.to_path_buf());
        Ok(())
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }

    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }

    /// Find symbols whose name contains `query`.
    pub fn search(&self, query: &str) -> Vec<&Symbol> {
        let q = query.to_lowercase();
        self.symbols
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&q))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn indexes_rust_function() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("lib.rs");
        std::fs::write(&file, "fn hello() {}\n").unwrap();
        let mut index = CodeIndex::new();
        index.index_file(&file).unwrap();
        assert!(!index.search("hello").is_empty());
    }
}
