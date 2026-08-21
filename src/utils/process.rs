//! Process execution utilities

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output, Stdio};
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::time::timeout;

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

        let mut child = command
            .spawn()
            .with_context(|| format!("Failed to spawn command: {cmd}"))?;

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
                    return Ok(Output {
                        status,
                        stdout,
                        stderr,
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

        let child = command
            .spawn()
            .with_context(|| format!("Failed to spawn command: {cmd}"))?;

        let output_future = child.wait_with_output();

        let output = timeout(timeout_duration, output_future)
            .await
            .with_context(|| format!("Command timed out after {timeout_duration:?}"))?
            .with_context(|| format!("Failed to wait for command: {cmd}"))?;

        Ok(output)
    }

    /// Execute a command and stream output line-by-line to the provided handlers
    pub async fn execute_streaming(
        cmd: &str,
        args: &[&str],
        cwd: Option<&Path>,
        env: Option<HashMap<String, String>>,
        timeout_duration: Duration,
        mut stdout_handler: impl FnMut(&str),
        mut stderr_handler: impl FnMut(&str),
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

        let mut child = command
            .spawn()
            .with_context(|| format!("Failed to spawn command: {cmd}"))?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let wait_future = async {
            if let Some(out) = stdout {
                let mut lines = BufReader::new(out).lines();
                while let Some(line) = lines
                    .next_line()
                    .await
                    .context("Failed to read command stdout")?
                {
                    stdout_handler(&line);
                }
            }
            if let Some(err) = stderr {
                let mut lines = BufReader::new(err).lines();
                while let Some(line) = lines
                    .next_line()
                    .await
                    .context("Failed to read command stderr")?
                {
                    stderr_handler(&line);
                }
            }
            let status = child
                .wait()
                .await
                .context("Failed to wait for command")?;
            anyhow::Ok(status)
        };

        timeout(timeout_duration, wait_future)
            .await
            .with_context(|| format!("Command timed out after {timeout_duration:?}"))?
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

    #[test]
    fn test_command_exists() {
        #[cfg(unix)]
        assert!(ProcessUtils::command_exists("ls") || ProcessUtils::command_exists("echo"));
        assert!(!ProcessUtils::command_exists("nonexistent_command_xyz123"));
    }

    #[tokio::test]
    async fn test_execute_simple() {
        let output = ProcessUtils::execute("echo", &["hello"], None, None, Duration::from_secs(5))
            .await
            .unwrap();

        assert!(output.status.success());
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
    }

    #[test]
    fn test_execute_sync_timeout() {
        #[cfg(unix)]
        {
            let err = ProcessUtils::execute_sync("sleep", &["5"], None, None, 1).unwrap_err();
            assert!(err.to_string().contains("timed out"));
        }
    }
}
