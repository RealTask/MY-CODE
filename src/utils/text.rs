//! Text processing utilities

use std::borrow::Cow;

/// Utility functions for text processing
pub struct TextUtils;

impl TextUtils {
    fn floor_char_boundary(text: &str, mut index: usize) -> usize {
        if index >= text.len() {
            return text.len();
        }
        while index > 0 && !text.is_char_boundary(index) {
            index -= 1;
        }
        index
    }

    /// Normalize line endings to Unix style (LF)
    pub fn normalize_line_endings(text: &str) -> Cow<str> {
        if text.contains("\r\n") {
            Cow::Owned(text.replace("\r\n", "\n"))
        } else if text.contains('\r') {
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
            Cow::Owned(format!("{text}\n"))
        }
    }

    /// Indent text by a given number of spaces
    pub fn indent(text: &str, spaces: usize) -> String {
        let prefix = " ".repeat(spaces);
        text.lines()
            .map(|line| format!("{prefix}{line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Dedent text by removing common leading whitespace
    pub fn dedent(text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        let min_indent = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
            .min()
            .unwrap_or(0);

        if min_indent == 0 {
            return text.to_string();
        }

        lines
            .iter()
            .map(|line| {
                if line.trim().is_empty() {
                    String::new()
                } else {
                    line.chars().skip(min_indent).collect::<String>()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Truncate text to a maximum length, adding ellipsis if truncated
    pub fn truncate(text: &str, max_len: usize) -> Cow<str> {
        if text.len() <= max_len {
            return Cow::Borrowed(text);
        }
        if max_len <= 3 {
            return Cow::Owned(".".repeat(max_len));
        }

        let limit = Self::floor_char_boundary(text, max_len.saturating_sub(3));
        let trunc_point = text[..limit].rfind(' ').unwrap_or(limit);
        let trunc_point = Self::floor_char_boundary(text, trunc_point);
        Cow::Owned(format!("{}...", &text[..trunc_point]))
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
        let offset = Self::floor_char_boundary(text, offset);
        Some(text[..offset].lines().count().max(1))
    }

    /// Extract a specific line from text (1-indexed)
    pub fn get_line(text: &str, line_number: usize) -> Option<&str> {
        if line_number == 0 {
            return None;
        }
        text.lines().nth(line_number - 1)
    }

    /// Extract lines within a range (1-indexed, inclusive)
    pub fn get_lines_range(text: &str, start: usize, end: usize) -> String {
        if start == 0 || end < start {
            return String::new();
        }
        text.lines()
            .skip(start - 1)
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
        assert_eq!(
            TextUtils::normalize_line_endings("hello\r\nworld"),
            "hello\nworld"
        );
        assert_eq!(
            TextUtils::normalize_line_endings("hello\nworld"),
            "hello\nworld"
        );
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
        assert_eq!(TextUtils::get_line(text, 0), None);
    }

    #[test]
    fn test_get_lines_range_invalid() {
        assert_eq!(TextUtils::get_lines_range("a\nb", 2, 1), "");
        assert_eq!(TextUtils::get_lines_range("a\nb", 0, 1), "");
    }

    #[test]
    fn test_truncate_utf8() {
        let text = "éééééééé";
        let truncated = TextUtils::truncate(text, 5);
        assert!(truncated.len() <= 5 || truncated.ends_with("..."));
    }
}
