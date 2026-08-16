//! Task representation

use uuid::Uuid;
use std::time::Instant;

/// A user task to be executed by the agent
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub created_at: Instant,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Default)]
pub enum TaskStatus {
    #[default]
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl Task {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description: description.into(),
            created_at: Instant::now(),
            status: TaskStatus::Pending,
        }
    }
    
    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
    }
    
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
    
    pub fn fail(&mut self) {
        self.status = TaskStatus::Failed;
    }
    
    pub fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
    }
    
    pub fn is_complete(&self) -> bool {
        matches!(self.status, TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled)
    }
}
