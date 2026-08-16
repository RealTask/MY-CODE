//! Tool result types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Result of a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// ID of the tool call this result corresponds to
    pub tool_call_id: String,

    /// Name of the tool that was executed
    pub tool_name: String,

    /// Whether the tool execution was successful
    pub success: bool,

    /// Output from the tool (stdout or result)
    pub output: String,

    /// Error message if execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Timestamp when the result was created
    #[serde(default)]
    pub timestamp: DateTime<Utc>,

    /// Optional metadata about the execution
    #[serde(default)]
    pub metadata: serde_json::Value,
}

impl ToolResult {
    /// Create a successful tool result
    pub fn success(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        output: impl Into<String>,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            success: true,
            output: output.into(),
            error: None,
            timestamp: Utc::now(),
            metadata: serde_json::Value::Null,
        }
    }

    /// Create a failed tool result
    pub fn error(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        error: impl Into<String>,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            success: false,
            output: String::new(),
            error: Some(error.into()),
            timestamp: Utc::now(),
            metadata: serde_json::Value::Null,
        }
    }

    /// Create a tool result with custom metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    /// Get a summary of the result for display
    pub fn summary(&self) -> String {
        if self.success {
            format!("✓ {} completed", self.tool_name)
        } else {
            format!(
                "✗ {} failed: {}",
                self.tool_name,
                self.error.as_deref().unwrap_or("unknown error")
            )
        }
    }

    /// Check if the result has an error
    pub fn has_error(&self) -> bool {
        !self.success || self.error.is_some()
    }
}

impl From<Result<String, String>> for ToolResult {
    fn from(result: Result<String, String>) -> Self {
        match result {
            Ok(output) => ToolResult::success("", "", output),
            Err(error) => ToolResult::error("", "", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_result() {
        let result = ToolResult::success("call_1", "read_file", "file contents");
        assert!(result.success);
        assert_eq!(result.output, "file contents");
        assert!(result.error.is_none());
    }

    #[test]
    fn test_error_result() {
        let result = ToolResult::error("call_1", "read_file", "file not found");
        assert!(!result.success);
        assert_eq!(result.error, Some("file not found".to_string()));
    }

    #[test]
    fn test_summary() {
        let success = ToolResult::success("c1", "test", "ok");
        assert!(success.summary().contains("completed"));

        let failure = ToolResult::error("c1", "test", "failed");
        assert!(failure.summary().contains("failed"));
    }
}
