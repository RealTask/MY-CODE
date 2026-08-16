//! Terminal execution tools

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

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
    
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    
    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code(),
        success: output.status.success(),
    })
}

/// Execute a command with timeout
pub fn execute_with_timeout(
    command: &str,
    args: &[&str],
    cwd: Option<&Path>,
    timeout_secs: u64,
) -> Result<CommandOutput> {
    use std::time::Duration;
    
    let mut cmd = Command::new(command);
    cmd.args(args);
    
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        use std::process::Child;
        
        // Spawn the process
        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        
        // Wait with timeout
        let duration = Duration::from_secs(timeout_secs);
        let handle = child.id();
        
        match child.wait_timeout(duration)? {
            Some(status) => Ok(CommandOutput {
                stdout: String::from_utf8_lossy(&child.stdout.unwrap_or_default()).to_string(),
                stderr: String::from_utf8_lossy(&child.stderr.unwrap_or_default()).to_string(),
                exit_code: status.code(),
                success: status.success(),
            }),
            None => {
                // Timeout - kill the process
                #[cfg(unix)]
                unsafe {
                    libc::kill(handle.unwrap() as i32, libc::SIGKILL);
                }
                
                anyhow::bail!("Command timed out after {} seconds", timeout_secs);
            }
        }
    }
    
    #[cfg(windows)]
    {
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        
        Ok(CommandOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            success: output.status.success(),
        })
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
        return "cmd.exe".to_string();
        #[cfg(unix)]
        return "/bin/sh".to_string();
    })
}

/// Terminal tools collection
pub struct TerminalTools;

impl TerminalTools {
    /// Execute a command
    pub fn run(command: &str, args: &[&str]) -> Result<CommandOutput> {
        execute_command(command, args, None, &[])
    }
    
    /// Execute in a specific directory
    pub fn run_in_dir(command: &str, args: &[&str], cwd: &Path) -> Result<CommandOutput> {
        execute_command(command, args, Some(cwd), &[])
    }
    
    /// Check if command exists
    pub fn exists(command: &str) -> bool {
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
        assert!(command_exists("dir"));
    }
}
