//! High-level coding agent

use anyhow::Result;
use crate::models::message::Message;
use crate::providers::provider::Provider;
use crate::tools::registry::ToolRegistry;
use crate::agent::task::Task;
use crate::agent::state::AgentState;
use crate::events::EventBus;

/// High-level coding agent
pub struct Agent {
    provider: Box<dyn Provider>,
    tool_registry: ToolRegistry,
    state: AgentState,
    event_bus: EventBus,
}

impl Agent {
    /// Create a new agent
    pub fn new(
        provider: Box<dyn Provider>,
        tool_registry: ToolRegistry,
        event_bus: EventBus,
    ) -> Self {
        Self {
            provider,
            tool_registry,
            state: AgentState::default(),
            event_bus,
        }
    }
    
    /// Get the current state
    pub fn state(&self) -> &AgentState {
        &self.state
    }
    
    /// Run a task
    pub async fn run(&mut self, task: Task) -> Result<String> {
        self.state.start_thinking();
        self.state.set_current_task(task.id.clone());
        
        // Placeholder - actual implementation will use orchestrator
        let response = format!("Task '{}' received", task.description);
        
        self.state.stop_thinking();
        self.state.clear_current_task();
        
        Ok(response)
    }
    
    /// Send a message and get a response
    pub async fn chat(&mut self, messages: Vec<Message>) -> Result<Message> {
        self.provider.chat(messages).await
    }
}
