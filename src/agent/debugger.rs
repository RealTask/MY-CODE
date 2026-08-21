//! Debugger agent

use anyhow::Result;

/// Investigates errors
#[derive(Debug, Default)]
pub struct Debugger;

impl Debugger {
    pub fn new() -> Self {
        Self
    }

    /// Debug an error
    pub async fn debug(&self, error: &str, context: &str) -> Result<String> {
        Ok(format!(
            "Debugging: {error} with context of {} bytes",
            context.len()
        ))
    }
}
