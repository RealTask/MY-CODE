//! Agent-specific errors

use thiserror::Error;

/// Errors related to agent operations
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Task execution failed: {0}")]
    TaskFailed(String),

    #[error("Planning failed: {0}")]
    PlanningFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Review failed: {0}")]
    ReviewFailed(String),

    #[error("Debugging failed: {0}")]
    DebugFailed(String),

    #[error("Model response error: {0}")]
    ModelResponse(String),

    #[error("Tool call error: {0}")]
    ToolCall(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("State error: {0}")]
    State(String),
}

pub type Result<T> = std::result::Result<T, AgentError>;
