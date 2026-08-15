//! Platform detection utilities

use std::env;

/// Information about the current platform
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub env: String,
    pub family: String,
}

impl Default for PlatformInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformInfo {
    /// Get platform information
    pub fn new() -> Self {
        Self {
            os: env::consts::OS.to_string(),
            arch: env::consts::ARCH.to_string(),
            env: env::consts::ENV.to_string(),
            family: env::consts::FAMILY.to_string(),
        }
    }

    /// Check if running on Linux
    pub fn is_linux(&self) -> bool {
        self.os == "linux"
    }

    /// Check if running on macOS
    pub fn is_macos(&self) -> bool {
        self.os == "macos"
    }

    /// Check if running on Windows
    pub fn is_windows(&self) -> bool {
        self.os == "windows"
    }

    /// Get the appropriate line ending for the platform
    pub fn line_ending(&self) -> &'static str {
        if self.is_windows() {
            "\r\n"
        } else {
            "\n"
        }
    }

    /// Get the path separator for the platform
    pub fn path_separator(&self) -> char {
        if self.is_windows() {
            '\\'
        } else {
            '/'
        }
    }

    /// Get the executable extension for the platform
    pub fn exe_extension(&self) -> &'static str {
        if self.is_windows() {
            ".exe"
        } else {
            ""
        }
    }

    /// Get the shell command for the platform
    pub fn shell_command(&self) -> (&'static str, &'static str) {
        if self.is_windows() {
            ("cmd", "/c")
        } else {
            ("sh", "-c")
        }
    }

    /// Check if running in a CI environment
    pub fn is_ci(&self) -> bool {
        env::var("CI").unwrap_or_default().to_lowercase() == "true"
            || env::var("GITHUB_ACTIONS").unwrap_or_default().to_lowercase() == "true"
            || env::var("GITLAB_CI").is_ok()
            || env::var("TRAVIS").unwrap_or_default().to_lowercase() == "true"
            || env::var("CIRCLECI").is_ok()
    }

    /// Check if running in a terminal with color support
    pub fn supports_color(&self) -> bool {
        // Check NO_COLOR environment variable
        if env::var("NO_COLOR").is_ok() {
            return false;
        }

        // Check FORCE_COLOR environment variable
        if env::var("FORCE_COLOR").is_ok() {
            return true;
        }

        // Check TERM for dumb terminals
        if let Ok(term) = env::var("TERM") {
            if term == "dumb" {
                return false;
            }
        }

        // On Windows, check for ANSI support
        if self.is_windows() {
            // Windows 10+ generally supports ANSI
            return true;
        }

        true
    }

    /// Get user agent string for HTTP requests
    pub fn user_agent(&self) -> String {
        format!(
            "my-code/{} ({}; {}; {})",
            env!("CARGO_PKG_VERSION"),
            self.os,
            self.arch,
            rust_version()
        )
    }
}

/// Get the Rust compiler version
fn rust_version() -> String {
    option_env!("CARGO_PKG_RUST_VERSION")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_info() {
        let info = PlatformInfo::new();
        
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
        
        // Exactly one OS should be true
        let os_count = [info.is_linux(), info.is_macos(), info.is_windows()]
            .iter()
            .filter(|&&b| b)
            .count();
        assert_eq!(os_count, 1);
    }

    #[test]
    fn test_line_ending() {
        let info = PlatformInfo::new();
        let ending = info.line_ending();
        
        if info.is_windows() {
            assert_eq!(ending, "\r\n");
        } else {
            assert_eq!(ending, "\n");
        }
    }
}
