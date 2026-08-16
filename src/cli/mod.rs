//! Command-line interface

pub mod args;
pub mod commands;
pub mod parser;
pub mod completions;

pub use args::CliArgs;
pub use commands::Command;
