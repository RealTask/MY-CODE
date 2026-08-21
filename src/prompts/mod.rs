//! System prompts and templates for different tasks.

/// Built-in system prompt used for coding sessions.
pub fn system_prompt() -> &'static str {
    "You are MY CODE, an AI-powered terminal coding agent for professional developers. \
     Inspect the project, plan carefully, make focused changes, run tools to verify, \
     and explain what you did."
}

/// Prompt used when creating an implementation plan.
pub fn plan_prompt(task: &str) -> String {
    format!("Create a concise, ordered implementation plan for the following task:\n\n{task}")
}

/// Prompt used when reviewing a diff.
pub fn review_prompt(diff: &str) -> String {
    format!("Review the following changes. Flag bugs, regressions, and missing tests.\n\n{diff}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompts_include_input() {
        assert!(plan_prompt("add auth").contains("add auth"));
        assert!(review_prompt("diff here").contains("diff here"));
        assert!(!system_prompt().is_empty());
    }
}
