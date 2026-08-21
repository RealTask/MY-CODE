//! Secret detection and permission management.

use regex::Regex;
use std::sync::OnceLock;

/// A detected secret-like string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretMatch {
    pub kind: &'static str,
    pub excerpt: String,
}

fn patterns() -> &'static Vec<(&'static str, Regex)> {
    static PATTERNS: OnceLock<Vec<(&'static str, Regex)>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            (
                "aws_access_key",
                Regex::new(r"AKIA[0-9A-Z]{16}").expect("valid regex"),
            ),
            (
                "generic_api_key",
                Regex::new(r"(?i)(api[_-]?key|secret|token)\s*[:=]\s*['\"][^'\"]{8,}['\"]")
                    .expect("valid regex"),
            ),
            (
                "private_key",
                Regex::new(r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----")
                    .expect("valid regex"),
            ),
        ]
    })
}

/// Scan `text` for likely secrets.
pub fn detect_secrets(text: &str) -> Vec<SecretMatch> {
    let mut matches = Vec::new();
    for (kind, regex) in patterns() {
        for mat in regex.find_iter(text) {
            matches.push(SecretMatch {
                kind,
                excerpt: mat.as_str().to_string(),
            });
        }
    }
    matches
}

/// Whether `text` appears to contain a secret.
pub fn contains_secret(text: &str) -> bool {
    !detect_secrets(text).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_aws_key() {
        let text = "key = AKIAIOSFODNN7EXAMPLE";
        assert!(contains_secret(text));
        assert_eq!(detect_secrets(text)[0].kind, "aws_access_key");
    }

    #[test]
    fn ignores_normal_code() {
        assert!(!contains_secret("let x = 1;"));
    }
}
