//! Diff generation and rendering.

use diffy::{create_patch, PatchFormatter};

/// Generate a unified diff between two texts.
pub fn unified(old: &str, new: &str, path: &str) -> String {
    let patch = create_patch(old, new);
    let formatted = PatchFormatter::new().fmt_patch(&patch).to_string();
    if formatted.trim().is_empty() || old == new {
        String::new()
    } else {
        format!("--- a/{path}\n+++ b/{path}\n{formatted}")
    }
}

/// Whether two texts differ.
pub fn differs(old: &str, new: &str) -> bool {
    old != new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_differences() {
        assert!(differs("a", "b"));
        assert!(!differs("a", "a"));
        let diff = unified("hello\n", "hello\nworld\n", "file.txt");
        assert!(diff.contains("world") || !diff.is_empty());
    }
}
