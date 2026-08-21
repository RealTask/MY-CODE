//! Agent orchestrator - controls the agent loop

use anyhow::Result;
use tracing::info;

use crate::agent::events::AgentEvent;
use crate::agent::task::Task;
use crate::events::EventBus;

/// Controls the agent execution loop
#[derive(Debug)]
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

        self.emit(AgentEvent::Started {
            task: task.description.clone(),
        });
        self.emit(AgentEvent::Thinking);

        Ok(())
    }

    fn emit(&self, event: AgentEvent) {
        self.event_bus.publish(event);
    }
}
