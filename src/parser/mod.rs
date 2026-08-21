//! Tree-sitter based code parsing for multiple languages.

use std::path::Path;

/// Languages the parser can identify.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Other,
}

/// Detect a language from a file path.
pub fn language_for_path(path: &Path) -> Language {
    match crate::tools::code::detect_language(path) {
        Some("rust") => Language::Rust,
        Some("python") => Language::Python,
        Some("typescript") => Language::TypeScript,
        Some("javascript") => Language::JavaScript,
        _ => Language::Other,
    }
}

/// Parse source into a simple line-oriented representation.
///
/// A full tree-sitter parse tree can be layered on later; this keeps the
/// public API stable and usable without extra native setup.
pub fn parse_source(source: &str) -> Vec<&str> {
    source.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_rust() {
        assert_eq!(language_for_path(Path::new("src/main.rs")), Language::Rust);
        assert_eq!(language_for_path(Path::new("app.py")), Language::Python);
    }
}
