//! Shell completions generation

use clap::Command;
use clap_complete::{Shell, Generator};

/// Generate completions for a specific shell
pub fn generate(shell: Shell, cmd: &mut Command) {
    let name = cmd.get_name().to_string();
    Generator::generate(&shell, cmd, name, &mut std::io::stdout());
}

/// Get all supported shells
pub fn supported_shells() -> &'static [Shell] {
    &[Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell]
}

/// Get the file extension for a shell
pub fn file_extension(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash => "bash",
        Shell::Zsh => "zsh",
        Shell::Fish => "fish",
        Shell::PowerShell => "ps1",
        _ => "txt",
    }
}
