//! Provider-specific errors

use thiserror::Error;

/// Errors related to model provider operations
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Provider not found: {0}")]
    NotFound(String),

    #[error("API request failed: {0}")]
    ApiRequest(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Model not available: {0}")]
    ModelNotAvailable(String),

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Stream error: {0}")]
    StreamError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, ProviderError>;
