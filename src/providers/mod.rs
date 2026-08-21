//! Provider configuration and abstractions

pub mod provider;
pub mod types;

pub use provider::{NullProvider, Provider, ProviderCapabilities};
pub use types::ProviderConfig;
