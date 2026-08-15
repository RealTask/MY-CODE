//! CLI argument definitions

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// MY CODE - AI-powered terminal coding agent
#[derive(Parser, Debug)]
#[command(name = "my-code")]
#[command(author = "MY CODE Team")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "AI-powered terminal coding agent", long_about = None)]
pub struct CliArgs {
    /// Working directory (defaults to current directory)
    #[arg(short = 'w', long, global = true)]
    pub workspace: Option<PathBuf>,

    /// Model to use (e.g., gpt-4, claude-3-5-sonnet)
    #[arg(short = 'm', long, global = true)]
    pub model: Option<String>,

    /// Provider to use (e.g., openai, anthropic, google)
    #[arg(short = 'p', long, global = true)]
    pub provider: Option<String>,

    /// Session ID to resume
    #[arg(short = 's', long, global = true)]
    pub session: Option<String>,

    /// Approval mode: auto, confirm, always
    #[arg(long, default_value = "confirm", global = true)]
    pub approval_mode: String,

    /// Enable verbose output
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,

    /// Enable debug output
    #[arg(long, global = true)]
    pub debug: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Output format: text, json
    #[arg(long, default_value = "text", global = true)]
    pub output_format: String,

    /// Run in non-interactive/headless mode
    #[arg(long, global = true)]
    pub non_interactive: bool,

    /// Configuration file path
    #[arg(short = 'c', long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start an interactive chat session
    Chat {
        /// Initial message or task
        #[arg(trailing_var_arg = true)]
        message: Vec<String>,
    },

    /// Create a plan for implementing a feature
    Plan {
        /// Feature or task description
        #[arg(required = true)]
        description: String,
    },

    /// Build or implement something
    Build {
        /// What to build
        #[arg(required = true)]
        description: String,
    },

    /// Review code changes
    Review {
        /// Path to review (defaults to staged changes)
        path: Option<String>,
    },

    /// Debug an issue
    Debug {
        /// Error or issue description
        #[arg(required = true)]
        description: String,
    },

    /// Run tests and fix failures
    Test {
        /// Test command or pattern
        pattern: Option<String>,
    },

    /// Explain code or concepts
    Explain {
        /// File or concept to explain
        #[arg(required = true)]
        target: String,
    },

    /// Show pending changes as a diff
    Diff,

    /// Undo the last change
    Undo {
        /// Number of changes to undo
        #[arg(default_value = "1")]
        count: usize,
    },

    /// Initialize MY CODE in a project
    Init,

    /// Show/edit configuration
    Config {
        /// Configuration key to get/set
        key: Option<String>,

        /// Value to set
        value: Option<String>,
    },

    /// Run diagnostics and check setup
    Doctor,

    /// List available sessions
    Sessions,

    /// Show version information
    Version,

    /// Generate shell completions
    Completions {
        /// Shell type: bash, zsh, fish, powershell
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

impl CliArgs {
    /// Check if any output should be quiet/minimal
    pub fn is_quiet(&self) -> bool {
        self.output_format == "json"
    }

    /// Get the effective log level
    pub fn log_level(&self) -> &str {
        if self.debug {
            "debug"
        } else if self.verbose {
            "info"
        } else {
            "warn"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_args() {
        let args = CliArgs::parse_from(["my-code", "chat", "hello"]);
        assert!(matches!(args.command, Some(Command::Chat { .. })));
    }

    #[test]
    fn test_parse_model_arg() {
        let args = CliArgs::parse_from(["my-code", "-m", "gpt-4", "doctor"]);
        assert_eq!(args.model, Some("gpt-4".to_string()));
    }

    #[test]
    fn test_parse_verbose() {
        let args = CliArgs::parse_from(["my-code", "-v", "doctor"]);
        assert!(args.verbose);
    }
}
