//! CLI commands definition

use clap::Subcommand;

/// Available commands
#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Start an interactive chat session
    Chat {
        /// Optional initial message
        #[arg(trailing_var_arg = true)]
        message: Vec<String>,
    },
    
    /// Create a plan for implementing a feature
    Plan {
        /// Description of what to plan
        #[arg(trailing_var_arg = true)]
        description: Vec<String>,
    },
    
    /// Build or generate code
    Build {
        /// Description of what to build
        #[arg(trailing_var_arg = true)]
        description: Vec<String>,
    },
    
    /// Review code changes
    Review {
        /// Specific files or PR to review
        files: Vec<String>,
    },
    
    /// Debug an issue
    Debug {
        /// Description of the issue
        #[arg(trailing_var_arg = true)]
        description: Vec<String>,
    },
    
    /// Run tests and fix failures
    Test {
        /// Specific test pattern
        pattern: Option<String>,
    },
    
    /// Explain code or concepts
    Explain {
        /// Path or concept to explain
        target: Option<String>,
    },
    
    /// Show diff of pending changes
    Diff,
    
    /// Undo the last change
    Undo {
        /// Number of changes to undo
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    
    /// Initialize MY CODE in a project
    Init,
    
    /// Manage configuration
    Config {
        #[clap(subcommand)]
        action: ConfigAction,
    },
    
    /// Run diagnostics
    Doctor,
    
    /// List and manage sessions
    Sessions {
        #[clap(subcommand)]
        action: SessionAction,
    },
    
    /// Print version information
    Version,
    
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
}

/// Configuration actions
#[derive(Subcommand, Debug, Clone)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Edit configuration
    Edit,
    /// Reset to defaults
    Reset,
    /// Set a configuration value
    Set { key: String, value: String },
    /// Get a configuration value
    Get { key: String },
}

/// Session actions
#[derive(Subcommand, Debug, Clone)]
pub enum SessionAction {
    /// List all sessions
    List,
    /// Show session details
    Show { id: String },
    /// Delete a session
    Delete { id: String },
    /// Clear all sessions
    Clear,
}
