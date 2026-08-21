//! Reviewer agent

use anyhow::Result;

/// Reviews generated changes
#[derive(Debug, Default)]
pub struct Reviewer;

impl Reviewer {
    pub fn new() -> Self {
        Self
    }

    /// Review changes
    pub async fn review(&self, diff: &str) -> Result<String> {
        Ok(format!("Review of {} bytes of changes", diff.len()))
    }
}
