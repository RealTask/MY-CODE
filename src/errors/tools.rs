//! Tool-specific errors

use thiserror::Error;

/// Errors related to tool operations
#[derive(Error, Debug)]
pub enum ToolError {
    #[error("Tool not found: {0}")]
    NotFound(String),

    #[error("Tool execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("File write error: {0}")]
    FileWrite(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Output limit exceeded")]
    OutputLimitExceeded,

    #[error("Path traversal detected")]
    PathTraversal,

    #[error("Outside workspace boundary")]
    OutsideWorkspace,
}

pub type Result<T> = std::result::Result<T, ToolError>;
