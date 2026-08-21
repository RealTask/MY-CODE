//! Executor agent

use anyhow::Result;

use crate::agent::step::Step;

/// Executes planned actions
#[derive(Debug, Default)]
pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    /// Execute a step
    pub async fn execute(&self, step: &Step) -> Result<String> {
        Ok(format!("Executed: {}", step.description))
    }
}
