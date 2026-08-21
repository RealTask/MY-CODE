//! Terminal execution tools

use anyhow::Result;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Result of command execution
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub success: bool,
}

/// Execute a command and capture output
pub fn execute_command(
    command: &str,
    args: &[&str],
    cwd: Option<&Path>,
    env: &[(&str, &str)],
) -> Result<CommandOutput> {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    for (key, value) in env {
        cmd.env(key, value);
    }

    let output = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output()?;

    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code(),
        success: output.status.success(),
    })
}

/// Execute a command with timeout. The process is killed if it exceeds `timeout_secs`.
pub fn execute_with_timeout(
    command: &str,
    args: &[&str],
    cwd: Option<&Path>,
    timeout_secs: u64,
) -> Result<CommandOutput> {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let mut stdout_handle = child.stdout.take();
    let mut stderr_handle = child.stderr.take();

    let stdout_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut handle) = stdout_handle.take() {
            let _ = handle.read_to_end(&mut buf);
        }
        buf
    });
    let stderr_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut handle) = stderr_handle.take() {
            let _ = handle.read_to_end(&mut buf);
        }
        buf
    });

    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    loop {
        match child.try_wait()? {
            Some(status) => {
                let stdout = stdout_thread.join().unwrap_or_default();
                let stderr = stderr_thread.join().unwrap_or_default();
                return Ok(CommandOutput {
                    stdout: String::from_utf8_lossy(&stdout).to_string(),
                    stderr: String::from_utf8_lossy(&stderr).to_string(),
                    exit_code: status.code(),
                    success: status.success(),
                });
            }
            None => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    anyhow::bail!("Command timed out after {timeout_secs} seconds");
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }
}

/// Run a command and stream output
pub fn run_interactive(command: &str, args: &[&str], cwd: Option<&Path>) -> Result<i32> {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    let status = cmd.status()?;
    Ok(status.code().unwrap_or(-1))
}

/// Check if a command exists
pub fn command_exists(command: &str) -> bool {
    which::which(command).is_ok()
}

/// Get the current shell
pub fn get_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| {
        #[cfg(windows)]
        {
            "cmd.exe".to_string()
        }
        #[cfg(not(windows))]
        {
            "/bin/sh".to_string()
        }
    })
}

/// Terminal tools collection
#[derive(Debug, Default, Clone, Copy)]
pub struct TerminalTools;

impl TerminalTools {
    pub fn new() -> Self {
        Self
    }

    /// Execute a command
    pub fn run(&self, command: &str, args: &[&str]) -> Result<CommandOutput> {
        execute_command(command, args, None, &[])
    }

    /// Execute in a specific directory
    pub fn run_in_dir(&self, command: &str, args: &[&str], cwd: &Path) -> Result<CommandOutput> {
        execute_command(command, args, Some(cwd), &[])
    }

    /// Check if command exists
    pub fn exists(&self, command: &str) -> bool {
        command_exists(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_echo() {
        let output = execute_command("echo", &["hello"], None, &[]).unwrap();
        assert!(output.success);
        assert!(output.stdout.contains("hello"));
    }

    #[test]
    fn test_command_exists() {
        #[cfg(unix)]
        assert!(command_exists("ls"));
        #[cfg(windows)]
        assert!(command_exists("cmd"));
    }

    #[test]
    fn test_timeout_kills_long_command() {
        #[cfg(unix)]
        {
            let err = execute_with_timeout("sleep", &["5"], None, 1).unwrap_err();
            assert!(err.to_string().contains("timed out"));
        }
    }
}
