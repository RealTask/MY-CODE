//! Git tools

use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// Git status information
#[derive(Debug, Clone, Default)]
pub struct GitStatus {
    pub branch: String,
    pub is_dirty: bool,
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    pub untracked_files: Vec<String>,
}

/// Get git status
pub fn get_status(repo_path: &Path) -> Result<GitStatus> {
    let mut status = GitStatus::default();

    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(repo_path)
        .output()?;

    if output.status.success() {
        status.branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }

    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo_path)
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.len() < 4 {
                continue;
            }

            let index_state = line.chars().next().unwrap_or(' ');
            let work_tree_state = line.chars().nth(1).unwrap_or(' ');
            let path = line[3..].to_string();

            if index_state != ' ' && index_state != '?' {
                status.staged_files.push(path.clone());
            }

            if work_tree_state != ' ' && work_tree_state != '?' {
                status.unstaged_files.push(path.clone());
            }

            if index_state == '?' {
                status.untracked_files.push(path);
            }
        }

        status.is_dirty = !status.staged_files.is_empty()
            || !status.unstaged_files.is_empty()
            || !status.untracked_files.is_empty();
    }

    Ok(status)
}

/// Get git diff
pub fn get_diff(repo_path: &Path, staged: bool) -> Result<String> {
    let mut cmd = Command::new("git");
    cmd.arg("diff");

    if staged {
        cmd.arg("--cached");
    }

    let output = cmd.current_dir(repo_path).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        anyhow::bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Get commit history
pub fn get_log(repo_path: &Path, max_commits: usize) -> Result<Vec<CommitInfo>> {
    let output = Command::new("git")
        .args([
            "log",
            "-n",
            &max_commits.to_string(),
            "--format=%H|%an|%ae|%ai|%s",
        ])
        .current_dir(repo_path)
        .output()?;

    let mut commits = Vec::new();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.splitn(5, '|').collect();
            if parts.len() >= 5 {
                commits.push(CommitInfo {
                    hash: parts[0].to_string(),
                    author_name: parts[1].to_string(),
                    author_email: parts[2].to_string(),
                    date: parts[3].to_string(),
                    message: parts[4].to_string(),
                });
            }
        }
    }

    Ok(commits)
}

/// Commit information
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub author_name: String,
    pub author_email: String,
    pub date: String,
    pub message: String,
}

/// Stage files
pub fn stage_files(repo_path: &Path, files: &[&str]) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("add").current_dir(repo_path);

    for file in files {
        cmd.arg(file);
    }

    let output = cmd.output()?;

    if output.status.success() {
        Ok(())
    } else {
        anyhow::bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Create a commit
pub fn commit(repo_path: &Path, message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(repo_path)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        anyhow::bail!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Check if directory is a git repo (including `.git` files used by worktrees)
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

/// Git tools collection
#[derive(Debug, Default, Clone, Copy)]
pub struct GitTools;

impl GitTools {
    pub fn new() -> Self {
        Self
    }

    /// Get repository status
    pub fn status(&self, repo_path: &Path) -> Result<GitStatus> {
        get_status(repo_path)
    }

    /// Get diff
    pub fn diff(&self, repo_path: &Path, staged: bool) -> Result<String> {
        get_diff(repo_path, staged)
    }

    /// Check if git repo
    pub fn is_repo(&self, path: &Path) -> bool {
        is_git_repo(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command as StdCommand;
    use tempfile::tempdir;

    #[test]
    fn test_is_git_repo() {
        let dir = tempdir().unwrap();
        assert!(!is_git_repo(dir.path()));

        let status = StdCommand::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output();

        if status.map(|o| o.status.success()).unwrap_or(false) {
            assert!(is_git_repo(dir.path()));
        }
    }
}
