//! Reviewer agent

use anyhow::Result;

/// Reviews generated changes
pub struct Reviewer;

impl Reviewer {
    pub fn new() -> Self {
        Self
    }
    
    /// Review changes
    pub async fn review(&self, diff: &str) -> Result<String> {
        // Placeholder - actual implementation will use LLM
        Ok(format!("Review of {} bytes of changes", diff.len()))
    }
}
