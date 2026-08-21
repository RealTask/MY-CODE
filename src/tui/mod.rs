//! Terminal user interface with streaming output.

/// High-level TUI handle. Rendering is optional; the CLI can run headless.
#[derive(Debug, Default)]
pub struct Tui {
    enabled: bool,
}

impl Tui {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn headless() -> Self {
        Self { enabled: false }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Print a status line when the TUI is enabled; otherwise no-op.
    pub fn status(&self, message: &str) {
        if self.enabled {
            println!("{message}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headless_is_disabled() {
        assert!(!Tui::headless().is_enabled());
        assert!(Tui::new().is_enabled());
    }
}
