//! Configuration validation.

use super::config::Config;

/// Errors found while validating a configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,
}

/// Validate a configuration, returning all problems found.
pub fn validate(config: &Config) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    if config.default_provider.trim().is_empty() {
        errors.push(ValidationError {
            message: "default_provider must not be empty".into(),
        });
    }

    let level = config.logging.level.to_lowercase();
    if !matches!(
        level.as_str(),
        "trace" | "debug" | "info" | "warn" | "error"
    ) {
        errors.push(ValidationError {
            message: format!("invalid log level '{}'", config.logging.level),
        });
    }

    if config.ui.max_width == 0 {
        errors.push(ValidationError {
            message: "ui.max_width must be greater than 0".into(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        assert!(validate(&Config::default()).is_ok());
    }

    #[test]
    fn rejects_empty_provider() {
        let mut config = Config::default();
        config.default_provider.clear();
        assert!(validate(&config).is_err());
    }
}
