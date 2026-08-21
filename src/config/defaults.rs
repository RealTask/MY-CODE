//! Default configuration values.

use super::config::Config;

/// Factory for default configuration.
pub struct Defaults;

impl Defaults {
    /// Built-in default configuration.
    pub fn config() -> Config {
        Config::default()
    }

    pub fn provider() -> &'static str {
        "openai"
    }

    pub fn theme() -> &'static str {
        "dark"
    }

    pub fn log_level() -> &'static str {
        "info"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_config() {
        let config = Defaults::config();
        assert_eq!(config.default_provider, Defaults::provider());
    }
}
