//! Text processing utilities

use std::borrow::Cow;

/// Utility functions for text processing
pub struct TextUtils;

impl TextUtils {
    /// Normalize line endings to Unix style (LF)
    pub fn normalize_line_endings(text: &str) -> Cow<str> {
        if text.contains("\r\n") {
            Cow::Owned(text.replace("\r\n", "\n"))
        } else if text.contains('\r') && !text.contains("\r\n") {
            Cow::Owned(text.replace('\r', "\n"))
        } else {
            Cow::Borrowed(text)
        }
    }

    /// Remove trailing whitespace from each line
    pub fn trim_trailing_whitespace(text: &str) -> String {
        text.lines()
            .map(|line| line.trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Ensure text ends with a newline
    pub fn ensure_trailing_newline(text: &str) -> Cow<str> {
        if text.is_empty() || text.ends_with('\n') {
            Cow::Borrowed(text)
        } else {
            Cow::Owned(format!("{}\n", text))
        }
    }

    /// Indent text by a given number of spaces
    pub fn indent(text: &str, spaces: usize) -> String {
        let prefix = " ".repeat(spaces);
        text.lines()
            .map(|line| format!("{}{}", prefix, line))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Dedent text by removing common leading whitespace
    pub fn dedent(text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        // Find minimum indentation (excluding empty lines)
        let min_indent = lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
            .min()
            .unwrap_or(0);

        if min_indent == 0 {
            return text.to_string();
        }

        lines
            .iter()
            .map(|line| {
                if line.len() >= min_indent {
                    &line[min_indent..]
                } else {
                    *line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Truncate text to a maximum length, adding ellipsis if truncated
    pub fn truncate(text: &str, max_len: usize) -> Cow<str> {
        if text.len() <= max_len {
            Cow::Borrowed(text)
        } else {
            // Try to break at word boundary
            let trunc_point = text[..max_len.saturating_sub(3)]
                .rfind(' ')
                .unwrap_or(max_len - 3);
            Cow::Owned(format!("{}...", &text[..trunc_point]))
        }
    }

    /// Split text into paragraphs (blocks separated by blank lines)
    pub fn split_paragraphs(text: &str) -> Vec<&str> {
        text.split("\n\n")
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect()
    }

    /// Count lines in text
    pub fn count_lines(text: &str) -> usize {
        if text.is_empty() {
            0
        } else {
            text.lines().count()
        }
    }

    /// Get line number for a byte offset
    pub fn byte_offset_to_line_number(text: &str, offset: usize) -> Option<usize> {
        if offset > text.len() {
            return None;
        }

        Some(text[..offset].lines().count() + 1)
    }

    /// Extract a specific line from text (1-indexed)
    pub fn get_line(text: &str, line_number: usize) -> Option<&str> {
        text.lines().nth(line_number - 1)
    }

    /// Extract lines within a range (1-indexed, inclusive)
    pub fn get_lines_range(text: &str, start: usize, end: usize) -> String {
        text.lines()
            .skip(start.saturating_sub(1))
            .take(end - start + 1)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Wrap text to a maximum width
    pub fn wrap_text(text: &str, width: usize) -> String {
        textwrap::fill(text, width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_line_endings() {
        assert_eq!(TextUtils::normalize_line_endings("hello\r\nworld"), "hello\nworld");
        assert_eq!(TextUtils::normalize_line_endings("hello\nworld"), "hello\nworld");
    }

    #[test]
    fn test_ensure_trailing_newline() {
        assert_eq!(TextUtils::ensure_trailing_newline("hello"), "hello\n");
        assert_eq!(TextUtils::ensure_trailing_newline("hello\n"), "hello\n");
    }

    #[test]
    fn test_indent() {
        let text = "line1\nline2";
        let indented = TextUtils::indent(text, 2);
        assert_eq!(indented, "  line1\n  line2");
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(TextUtils::count_lines("a\nb\nc"), 3);
        assert_eq!(TextUtils::count_lines(""), 0);
    }

    #[test]
    fn test_get_line() {
        let text = "first\nsecond\nthird";
        assert_eq!(TextUtils::get_line(text, 1), Some("first"));
        assert_eq!(TextUtils::get_line(text, 2), Some("second"));
        assert_eq!(TextUtils::get_line(text, 4), None);
    }
}
