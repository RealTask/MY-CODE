//! Planning agent

use crate::agent::step::Step;

/// Creates implementation plans
pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }
    
    /// Create a plan from a description
    pub fn create_plan(&self, description: &str) -> Vec<Step> {
        // Placeholder - actual implementation will use LLM
        vec![Step::new(format!("Plan step for: {}", description))]
    }
}
