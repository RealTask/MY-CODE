//! Tool call types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A tool/function call from the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique identifier for this tool call
    #[serde(default)]
    pub id: String,

    /// Type of call (usually "function")
    #[serde(default = "default_call_type")]
    pub call_type: String,

    /// Function to call
    pub function: ToolFunction,
}

fn default_call_type() -> String {
    "function".to_string()
}

impl ToolCall {
    /// Create a new tool call
    pub fn new(name: impl Into<String>, arguments: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            call_type: "function".to_string(),
            function: ToolFunction {
                name: name.into(),
                arguments,
            },
        }
    }

    /// Create a tool call with a specific ID
    pub fn with_id(id: impl Into<String>, name: impl Into<String>, arguments: serde_json::Value) -> Self {
        Self {
            id: id.into(),
            call_type: "function".to_string(),
            function: ToolFunction {
                name: name.into(),
                arguments,
            },
        }
    }

    /// Get the function name
    pub fn name(&self) -> &str {
        &self.function.name
    }

    /// Get the function arguments
    pub fn arguments(&self) -> &serde_json::Value {
        &self.function.arguments
    }

    /// Parse arguments as a specific type
    pub fn arguments_as<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.function.arguments.clone())
    }
}

/// A function definition for tool calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    /// Name of the function
    pub name: String,

    /// Arguments as JSON value
    pub arguments: serde_json::Value,
}

impl ToolFunction {
    /// Create a new function definition
    pub fn new(name: impl Into<String>, arguments: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            arguments,
        }
    }
}

/// Schema for a tool that can be called
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    /// Name of the tool
    pub name: String,

    /// Description of what the tool does
    pub description: String,

    /// Parameter schema (JSON Schema format)
    pub parameters: serde_json::Value,
}

impl ToolSchema {
    /// Create a new tool schema
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: serde_json::Value,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters,
        }
    }

    /// Create a simple tool schema with no parameters
    pub fn simple(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_call_creation() {
        let call = ToolCall::new("read_file", serde_json::json!({"path": "test.rs"}));
        assert_eq!(call.name(), "read_file");
        assert!(!call.id.is_empty());
    }

    #[test]
    fn test_tool_call_with_id() {
        let call = ToolCall::with_id("custom_id", "write_file", serde_json::json!({}));
        assert_eq!(call.id, "custom_id");
    }

    #[test]
    fn test_arguments_parsing() {
        let call = ToolCall::new(
            "test_fn",
            serde_json::json!({"value": 42, "name": "test"}),
        );

        #[derive(Deserialize, Debug, PartialEq)]
        struct TestArgs {
            value: i32,
            name: String,
        }

        let args: TestArgs = call.arguments_as().unwrap();
        assert_eq!(args.value, 42);
        assert_eq!(args.name, "test");
    }
}
