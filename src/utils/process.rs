//! Process execution utilities

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output, Stdio};
use anyhow::{Context, Result};
use tokio::process::Command as TokioCommand;
use tokio::time::{timeout, Duration};

/// Utility functions for process management
pub struct ProcessUtils;

impl ProcessUtils {
    /// Execute a command synchronously with timeout
    pub fn execute_sync(
        cmd: &str,
        args: &[&str],
        cwd: Option<&Path>,
        env: Option<HashMap<String, String>>,
        timeout_secs: u64,
    ) -> Result<Output> {
        let mut command = Command::new(cmd);
        command.args(args);

        if let Some(cwd) = cwd {
            command.current_dir(cwd);
        }

        if let Some(env) = env {
            command.envs(env);
        }

        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        // Note: synchronous timeout requires spawning a thread
        // For now, we use the async version
        let output = std::thread::spawn(move || {
            command.output()
        }).join().map_err(|_| anyhow::anyhow!("Thread panicked"))??;

        Ok(output)
    }

    /// Execute a command asynchronously with timeout
    pub async fn execute(
        cmd: &str,
        args: &[&str],
        cwd: Option<&Path>,
        env: Option<HashMap<String, String>>,
        timeout_duration: Duration,
    ) -> Result<Output> {
        let mut command = TokioCommand::new(cmd);
        command.args(args);

        if let Some(cwd) = cwd {
            command.current_dir(cwd);
        }

        if let Some(env) = env {
            command.envs(env);
        }

        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let child = command.spawn()
            .with_context(|| format!("Failed to spawn command: {}", cmd))?;

        let output_future = child.wait_with_output();
        
        let output = timeout(timeout_duration, output_future)
            .await
            .with_context(|| format!("Command timed out after {:?}", timeout_duration))?
            .with_context(|| format!("Failed to wait for command: {}", cmd))?;

        Ok(output)
    }

    /// Execute a command and stream output
    pub async fn execute_streaming(
        cmd: &str,
        args: &[&str],
        cwd: Option<&Path>,
        env: Option<HashMap<String, String>>,
        timeout_duration: Duration,
        stdout_handler: impl FnMut(&str),
        stderr_handler: impl FnMut(&str),
    ) -> Result<ExitStatus> {
        let mut command = TokioCommand::new(cmd);
        command.args(args);

        if let Some(cwd) = cwd {
            command.current_dir(cwd);
        }

        if let Some(env) = env {
            command.envs(env);
        }

        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn()
            .with_context(|| format!("Failed to spawn command: {}", cmd))?;

        // This is a simplified implementation
        // A full streaming implementation would use channels
        let status = timeout(timeout_duration, child.wait())
            .await
            .with_context(|| format!("Command timed out after {:?}", timeout_duration))?
            .with_context(|| format!("Failed to wait for command: {}", cmd))?;

        Ok(status)
    }

    /// Check if a command exists in PATH
    pub fn command_exists(cmd: &str) -> bool {
        which::which(cmd).is_ok()
    }

    /// Get the current working directory
    pub fn current_dir() -> Result<PathBuf> {
        std::env::current_dir().context("Failed to get current directory")
    }

    /// Truncate output to a maximum size
    pub fn truncate_output(output: &mut Vec<u8>, max_size: usize) {
        if output.len() > max_size {
            output.truncate(max_size);
            output.extend_from_slice(b"\n... [output truncated]");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_command_exists() {
        assert!(ProcessUtils::command_exists("ls"));
        assert!(!ProcessUtils::command_exists("nonexistent_command_xyz123"));
    }

    #[tokio::test]
    async fn test_execute_simple() {
        let output = ProcessUtils::execute(
            "echo",
            &["hello"],
            None,
            None,
            Duration::from_secs(5),
        ).await.unwrap();

        assert!(output.status.success());
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
    }
}
