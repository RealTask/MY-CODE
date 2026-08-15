//! Core application error types

use thiserror::Error;

/// Application-level errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Tool error: {0}")]
    Tool(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("Workspace error: {0}")]
    Workspace(String),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Context error: {0}")]
    Context(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Edit error: {0}")]
    Edit(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for application errors
pub type Result<T> = std::result::Result<T, AppError>;

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}
