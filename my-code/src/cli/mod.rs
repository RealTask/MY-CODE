//! Command-line interface

pub mod args;
pub mod commands;
pub mod parser;
pub mod completions;

pub use args::CliArgs;
pub use commands::Command;
pub use parser::parse_args;
pub use completions::generate_completions;
