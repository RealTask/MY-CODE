//! Application lifecycle hooks

use anyhow::Result;
use tracing::info;

/// Handles application startup and shutdown hooks
pub struct Lifecycle;

impl Lifecycle {
    /// Run startup hooks
    pub fn startup() -> Result<()> {
        info!("MY CODE starting up");
        Ok(())
    }
    
    /// Run shutdown hooks
    pub fn shutdown() -> Result<()> {
        info!("MY CODE shutting down");
        Ok(())
    }
}
