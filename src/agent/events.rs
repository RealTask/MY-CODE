//! Agent events

/// Events emitted by the agent
#[derive(Debug, Clone)]
pub enum AgentEvent {
    Started { task: String },
    Thinking,
    PlanCreated { steps: usize },
    ToolStarted { name: String },
    ToolFinished { name: String, success: bool },
    FileChanged { path: String },
    CommandStarted { command: String },
    CommandFinished { command: String, exit_code: Option<i32> },
    TestStarted { name: String },
    TestFinished { name: String, passed: bool },
    ApprovalRequired { description: String },
    CheckpointCreated { id: String },
    Finished { success: bool },
    Error { message: String },
}

impl AgentEvent {
    pub fn description(&self) -> &'static str {
        match self {
            AgentEvent::Started { .. } => "Agent started",
            AgentEvent::Thinking => "Thinking...",
            AgentEvent::PlanCreated { .. } => "Plan created",
            AgentEvent::ToolStarted { .. } => "Tool started",
            AgentEvent::ToolFinished { .. } => "Tool finished",
            AgentEvent::FileChanged { .. } => "File changed",
            AgentEvent::CommandStarted { .. } => "Command started",
            AgentEvent::CommandFinished { .. } => "Command finished",
            AgentEvent::TestStarted { .. } => "Test started",
            AgentEvent::TestFinished { .. } => "Test finished",
            AgentEvent::ApprovalRequired { .. } => "Approval required",
            AgentEvent::CheckpointCreated { .. } => "Checkpoint created",
            AgentEvent::Finished { .. } => "Finished",
            AgentEvent::Error { .. } => "Error",
        }
    }
}
