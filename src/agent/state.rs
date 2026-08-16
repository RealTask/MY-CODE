//! Agent state tracking

/// Current state of the agent
#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub is_thinking: bool,
    pub current_task_id: Option<String>,
    pub pending_tool_calls: usize,
    pub error_count: usize,
}

impl AgentState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn start_thinking(&mut self) {
        self.is_thinking = true;
    }
    
    pub fn stop_thinking(&mut self) {
        self.is_thinking = false;
    }
    
    pub fn set_current_task(&mut self, task_id: String) {
        self.current_task_id = Some(task_id);
    }
    
    pub fn clear_current_task(&mut self) {
        self.current_task_id = None;
    }
    
    pub fn increment_tool_calls(&mut self) {
        self.pending_tool_calls += 1;
    }
    
    pub fn decrement_tool_calls(&mut self) {
        if self.pending_tool_calls > 0 {
            self.pending_tool_calls -= 1;
        }
    }
    
    pub fn record_error(&mut self) {
        self.error_count += 1;
    }
    
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
