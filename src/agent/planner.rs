//! Planning agent

use crate::agent::step::Step;

/// Creates implementation plans
#[derive(Debug, Default)]
pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }

    /// Create a plan from a description
    pub fn create_plan(&self, description: &str) -> Vec<Step> {
        vec![Step::new(format!("Plan step for: {description}"))]
    }
}
