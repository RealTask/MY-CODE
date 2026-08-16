//! Step representation

use uuid::Uuid;

/// An individual execution step within a task
#[derive(Debug, Clone)]
pub struct Step {
    pub id: String,
    pub description: String,
    pub status: StepStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub enum StepStatus {
    #[default]
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

impl Step {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description: description.into(),
            status: StepStatus::Pending,
            error: None,
        }
    }
    
    pub fn start(&mut self) {
        self.status = StepStatus::Running;
    }
    
    pub fn complete(&mut self) {
        self.status = StepStatus::Completed;
    }
    
    pub fn fail(&mut self, error: impl Into<String>) {
        self.status = StepStatus::Failed;
        self.error = Some(error.into());
    }
    
    pub fn skip(&mut self) {
        self.status = StepStatus::Skipped;
    }
}
