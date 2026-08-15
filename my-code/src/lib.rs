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
//! - **CLI**: Command-line interface and argument parsing
//! - **App**: Application lifecycle and state management
//! - **Agent**: AI agent orchestration and task execution
//! - **Models**: Message and response types for model interactions
//! - **Providers**: LLM provider abstractions and implementations
//! - **Tools**: Filesystem, terminal, search, git, and code analysis tools
//! - **Workspace**: Project detection, scanning, and understanding
//! - **Context**: Context collection, ranking, and budget management
//! - **Index**: Code indexing and symbol tracking
//! - **Parser**: Tree-sitter based code parsing
//! - **Edit**: Patch generation and application
//! - **Diff**: Diff generation and rendering
//! - **Git**: Git repository operations
//! - **Sandbox**: Security policies and command restrictions
//! - **Security**: Secret detection and permission management
//! - **Sessions**: Session management and persistence
//! - **Memory**: Project and conversation memory
//! - **Checkpoints**: File checkpoints and undo functionality
//! - **Prompts**: System prompts and templates
//! - **Commands**: High-level command implementations
//! - **TUI**: Terminal user interface
//! - **Config**: Configuration loading and validation
//! - **Database**: SQLite persistence layer
//! - **Plugins**: Plugin system for extensibility
//! - **Events**: Event bus for decoupled communication
//! - **Telemetry**: Metrics and tracing
//! - **Errors**: Error types and handling
//! - **Utils**: Utility functions

pub mod app;
pub mod cli;
pub mod agent;
pub mod models;
pub mod providers;
pub mod tools;
pub mod workspace;
pub mod context;
pub mod index;
pub mod parser;
pub mod edit;
pub mod diff;
pub mod git;
pub mod sandbox;
pub mod security;
pub mod sessions;
pub mod memory;
pub mod checkpoints;
pub mod prompts;
pub mod commands;
pub mod tui;
pub mod config;
pub mod database;
pub mod plugins;
pub mod events;
pub mod telemetry;
pub mod errors;
pub mod utils;

// Re-export commonly used types
pub use app::application::Application;
pub use cli::commands::Command;
pub use config::config::Config;
pub use errors::app::{AppError, Result};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = "my-code";
