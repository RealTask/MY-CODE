//! Application lifecycle coordinator

use anyhow::Result;
use crate::config::Config;
use crate::sessions::SessionManager;
use crate::database::Database;
use crate::events::EventBus;

/// Central application coordinator
pub struct Application {
    config: Config,
    session_manager: SessionManager,
    database: Database,
    event_bus: EventBus,
}

impl Application {
    /// Create a new application instance
    pub fn new(config: Config) -> Result<Self> {
        let database = Database::new()?;
        let session_manager = SessionManager::new(&database)?;
        let event_bus = EventBus::new();
        
        Ok(Self {
            config,
            session_manager,
            database,
            event_bus,
        })
    }
    
    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Get the session manager
    pub fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }
    
    /// Get the database
    pub fn database(&self) -> &Database {
        &self.database
    }
    
    /// Get the event bus
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }
    
    /// Run the application with the given command
    pub async fn run(&mut self) -> Result<i32> {
        // Placeholder - actual implementation will be in CLI layer
        Ok(0)
    }
    
    /// Shutdown the application gracefully
    pub async fn shutdown(&mut self) -> Result<()> {
        self.database.close()?;
        Ok(())
    }
}
