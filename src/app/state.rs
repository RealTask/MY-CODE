//! Global runtime state

use crate::config::Config;
use crate::workspace::Workspace;
use crate::sessions::Session;
use crate::agent::AgentState;

/// Global runtime state
pub struct AppState {
    config: Config,
    workspace: Option<Workspace>,
    session: Option<Session>,
    agent_state: AgentState,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: Config) -> Self {
        Self {
            config,
            workspace: None,
            session: None,
            agent_state: AgentState::default(),
        }
    }
    
    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Get the current workspace
    pub fn workspace(&self) -> Option<&Workspace> {
        self.workspace.as_ref()
    }
    
    /// Set the workspace
    pub fn set_workspace(&mut self, workspace: Workspace) {
        self.workspace = Some(workspace);
    }
    
    /// Get the current session
    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }
    
    /// Set the session
    pub fn set_session(&mut self, session: Session) {
        self.session = Some(session);
    }
    
    /// Get the agent state
    pub fn agent_state(&self) -> &AgentState {
        &self.agent_state
    }
    
    /// Get mutable agent state
    pub fn agent_state_mut(&mut self) -> &mut AgentState {
        &mut self.agent_state
    }
}
