//! Agent-specific errors with comprehensive error handling
//!
//! This module provides detailed error types for all agent operations,
//! including task execution, planning, tool calls, and provider interactions.

use thiserror::Error;
use crate::errors::provider::ProviderError;

/// Errors related to agent operations
#[derive(Error, Debug)]
pub enum AgentError {
    /// Task execution failed
    #[error("Task execution failed: {0}")]
    TaskFailed(String),

    /// Planning phase failed
    #[error("Planning failed: {0}")]
    PlanningFailed(String),

    /// Execution phase failed
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    /// Review phase failed
    #[error("Review failed: {0}")]
    ReviewFailed(String),

    /// Debugging phase failed
    #[error("Debugging failed: {0}")]
    DebugFailed(String),

    /// Invalid or unexpected model response
    #[error("Model response error: {0}")]
    ModelResponse(String),

    /// Tool call execution error
    #[error("Tool call error: {0}")]
    ToolCall(String),

    /// Validation error during task processing
    #[error("Validation error: {0}")]
    Validation(String),

    /// Operation timed out
    #[error("Timeout error: {0}")]
    Timeout(String),

    /// Invalid agent state transition
    #[error("State error: {0}")]
    State(String),

    /// Provider error wrapped as agent error
    #[error("Provider error: {0}")]
    ProviderError(String),

    /// Maximum retries exceeded
    #[error("Maximum retries ({max_retries}) exceeded for operation: {operation}")]
    MaxRetriesExceeded {
        operation: String,
        max_retries: u32,
    },

    /// Context window exceeded
    #[error("Context window exceeded: used {used} tokens, limit is {limit}")]
    ContextWindowExceeded {
        used: usize,
        limit: usize,
    },

    /// Tool call limit exceeded
    #[error("Tool call limit exceeded: made {count} calls, limit is {limit}")]
    ToolCallLimitExceeded {
        count: usize,
        limit: usize,
    },

    /// Missing required field in response
    #[error("Missing required field: {field}")]
    MissingField {
        field: String,
    },

    /// Invalid tool arguments
    #[error("Invalid tool arguments for '{tool}': {reason}")]
    InvalidToolArguments {
        tool: String,
        reason: String,
    },

    /// Permission denied for operation
    #[error("Permission denied: {operation}")]
    PermissionDenied {
        operation: String,
    },

    /// Session expired or invalid
    #[error("Session error: {0}")]
    SessionError(String),
}

impl From<ProviderError> for AgentError {
    fn from(err: ProviderError) -> Self {
        AgentError::ProviderError(err.to_string())
    }
}

impl From<anyhow::Error> for AgentError {
    fn from(err: anyhow::Error) -> Self {
        AgentError::TaskFailed(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AgentError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_error_display() {
        let err = AgentError::TaskFailed("Test failure".to_string());
        assert_eq!(format!("{}", err), "Task execution failed: Test failure");
    }

    #[test]
    fn test_context_window_exceeded() {
        let err = AgentError::ContextWindowExceeded {
            used: 150_000,
            limit: 128_000,
        };
        assert!(err.to_string().contains("150000"));
        assert!(err.to_string().contains("128000"));
    }

    #[test]
    fn test_max_retries_exceeded() {
        let err = AgentError::MaxRetriesExceeded {
            operation: "API call".to_string(),
            max_retries: 3,
        };
        assert!(err.to_string().contains("API call"));
        assert!(err.to_string().contains("3"));
    }

    #[test]
    fn test_error_from_provider() {
        let provider_err = ProviderError::ApiRequest("Connection failed".to_string());
        let agent_err: AgentError = provider_err.into();
        assert!(matches!(agent_err, AgentError::ProviderError(_)));
    }
}
