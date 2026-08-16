//! Agent orchestrator - controls the agent loop

use anyhow::Result;
use tracing::info;
use crate::agent::task::Task;
use crate::agent::step::Step;
use crate::agent::events::AgentEvent;
use crate::events::EventBus;

/// Controls the agent execution loop
pub struct Orchestrator {
    event_bus: EventBus,
}

impl Orchestrator {
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
    
    /// Execute a task through the full pipeline
    pub async fn execute(&self, task: &Task) -> Result<()> {
        info!("Executing task: {}", task.description);
        
        self.emit(AgentEvent::Started { task: task.description.clone() });
        self.emit(AgentEvent::Thinking);
        
        // Pipeline:
        // USER -> ANALYSIS -> PLAN -> CONTEXT -> MODEL -> TOOL CALL -> RESULT -> VALIDATION -> RESPONSE
        
        Ok(())
    }
    
    fn emit(&self, event: AgentEvent) {
        self.event_bus.publish(event);
    }
}
