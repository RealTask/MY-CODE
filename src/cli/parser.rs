//! CLI argument parser

use clap::{CommandFactory, Parser};

use crate::cli::args::{CliArgs, Command};

/// Parse command-line arguments
pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

/// Get the command from parsed args
pub fn get_command(args: &CliArgs) -> Option<&Command> {
    args.command.as_ref()
}

/// Generate shell completions
pub fn generate_completions(shell: clap_complete::Shell, cmd: &mut clap::Command) {
    let name = cmd.get_name().to_string();
    clap_complete::generate(shell, cmd, name, &mut std::io::stdout());
}

/// Create the root command for clap
pub fn create_command() -> clap::Command {
    CliArgs::command()
}
