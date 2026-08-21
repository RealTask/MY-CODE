//! MY CODE - AI-powered terminal coding agent
//!
//! A professional developer tool that allows developers to interact with their codebase
//! using natural language. MY CODE inspects projects, understands context, plans work,
//! makes changes, runs tools, verifies results, and explains what happened.
//!
//! # Architecture
//!
//! MY CODE uses a layered architecture with the following core components:
//!
//! - **CLI**: Command-line interface and argument parsing with shell completions
//! - **App**: Application lifecycle and state management
//! - **Agent**: AI agent orchestration including planner, executor, reviewer, and debugger
//! - **Models**: Message and response types for LLM interactions
//! - **Providers**: LLM provider abstractions (OpenAI, Anthropic, Google, etc.)
//! - **Tools**: Filesystem, terminal, search, git, and code analysis tools
//! - **Workspace**: Project detection, scanning, and understanding
//! - **Context**: Context collection, ranking, and budget management
//! - **Index**: Code indexing and symbol tracking using tree-sitter
//! - **Parser**: Tree-sitter based code parsing for multiple languages
//! - **Edit**: Patch generation and application for safe code modifications
//! - **Diff**: Diff generation and rendering with syntax highlighting
//! - **Git**: Git repository operations and change tracking
//! - **Sandbox**: Security policies and command restrictions
//! - **Security**: Secret detection and permission management
//! - **Sessions**: Session management and persistence
//! - **Memory**: Project and conversation memory for continuity
//! - **Checkpoints**: File checkpoints and undo functionality
//! - **Prompts**: System prompts and templates for different tasks
//! - **Commands**: High-level command implementations
//! - **TUI**: Terminal user interface with streaming output
//! - **Config**: Configuration loading and validation
//! - **Database**: SQLite persistence layer for sessions and memory
//! - **Plugins**: Plugin system for extensibility
//! - **Events**: Event bus for decoupled communication
//! - **Telemetry**: Metrics and tracing for debugging
//! - **Errors**: Comprehensive error types and handling
//! - **Utils**: Utility functions for common operations
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use my_code::{Application, Config, Command};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config::default();
//!     let mut app = Application::new(config)?;
//!     app.run(Command::Chat {
//!         message: vec!["Build authentication".to_string()],
//!     })
//!     .await?;
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
//! - 🤖 **Intelligent Agent** - Plans, executes, and reviews code changes
//! - 🔍 **Workspace Understanding** - Detects languages, frameworks, and project structure
//! - 📁 **Smart Context** - Selects relevant files and symbols automatically
//! - 🛠️ **Powerful Tools** - Filesystem, terminal, search, git, and code analysis
//! - 🔒 **Security First** - Permission system, sandboxing, and approval workflows
//! - 💾 **Sessions & Memory** - Resume work, track history, learn from context
//! - 🎨 **Beautiful TUI** - Polished terminal interface with streaming output
//! - 🔌 **Extensible** - Plugin system for custom tools and providers
//! - 🌐 **Multi-Provider** - OpenAI, Anthropic, Google, and compatible APIs

pub mod agent;
pub mod app;
pub mod checkpoints;
pub mod cli;
pub mod commands;
pub mod config;
pub mod context;
pub mod database;
pub mod diff;
pub mod edit;
pub mod errors;
pub mod events;
pub mod git;
pub mod index;
pub mod memory;
pub mod models;
pub mod parser;
pub mod plugins;
pub mod prompts;
pub mod providers;
pub mod sandbox;
pub mod security;
pub mod sessions;
pub mod telemetry;
pub mod tools;
pub mod tui;
pub mod utils;
pub mod workspace;

// Re-export commonly used types for convenience
pub use app::application::Application;
pub use cli::commands::Command;
pub use config::config::Config;
pub use errors::app::{AppError, Result};

/// Library version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = "my-code";

/// Default context window size in tokens
pub const DEFAULT_CONTEXT_WINDOW: usize = 128_000;

/// Default maximum output tokens
pub const DEFAULT_MAX_OUTPUT_TOKENS: usize = 4_096;

/// Maximum number of consecutive tool calls allowed
pub const MAX_TOOL_CALLS_PER_TURN: usize = 50;

/// Default approval mode
pub const DEFAULT_APPROVAL_MODE: &str = "confirm";
