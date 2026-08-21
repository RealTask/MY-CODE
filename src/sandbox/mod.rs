//! Security policies and command restrictions.

use serde::{Deserialize, Serialize};

/// Policy controlling what the agent is allowed to do on the host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxPolicy {
    /// Enable sandbox enforcement.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Allow outbound network access from tools.
    #[serde(default)]
    pub allow_network: bool,
    /// Allow shell / terminal command execution.
    #[serde(default = "default_true")]
    pub allow_shell: bool,
    /// Allow writes outside the workspace.
    #[serde(default)]
    pub allow_outside_workspace: bool,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            allow_network: false,
            allow_shell: true,
            allow_outside_workspace: false,
        }
    }
}

impl SandboxPolicy {
    /// Restrictive policy that denies shell and network.
    pub fn strict() -> Self {
        Self {
            enabled: true,
            allow_network: false,
            allow_shell: false,
            allow_outside_workspace: false,
        }
    }

    /// Whether a command may run under this policy.
    pub fn allows_shell(&self) -> bool {
        !self.enabled || self.allow_shell
    }
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_allows_shell_but_not_network() {
        let policy = SandboxPolicy::default();
        assert!(policy.allows_shell());
        assert!(!policy.allow_network);
    }
}
