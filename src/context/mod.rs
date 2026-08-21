//! Context collection, ranking, and budget management.

use serde::{Deserialize, Serialize};

/// Configuration for how much project context is sent to the model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextConfig {
    /// Maximum tokens of context to include.
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    /// Maximum number of files to attach.
    #[serde(default = "default_max_files")]
    pub max_files: usize,
    /// Include git diff in context.
    #[serde(default = "default_true")]
    pub include_diff: bool,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_tokens: default_max_tokens(),
            max_files: default_max_files(),
            include_diff: true,
        }
    }
}

fn default_max_tokens() -> usize {
    128_000
}

fn default_max_files() -> usize {
    50
}

fn default_true() -> bool {
    true
}

/// A single piece of context selected for a prompt.
#[derive(Debug, Clone)]
pub struct ContextItem {
    pub path: String,
    pub content: String,
    pub tokens: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_budget() {
        let cfg = ContextConfig::default();
        assert_eq!(cfg.max_tokens, 128_000);
        assert!(cfg.include_diff);
    }
}
