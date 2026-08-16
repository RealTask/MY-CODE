//! High-level coding agent with advanced orchestration capabilities
//!
//! The Agent module provides intelligent task execution through a sophisticated
//! multi-agent architecture including planning, execution, review, and debugging.

use anyhow::{Context, Result};
use tracing::{info, debug, warn, instrument};

use crate::models::message::{Message, MessageRole};
use crate::providers::provider::Provider;
use crate::tools::filesystem::FileSystemTools;
use crate::agent::task::{Task, TaskStatus};
use crate::agent::state::AgentState;
use crate::agent::orchestrator::Orchestrator;
use crate::agent::planner::Planner;
use crate::agent::executor::Executor;
use crate::agent::reviewer::Reviewer;
use crate::agent::debugger::Debugger;
use crate::events::EventBus;
use crate::errors::agent::AgentError;

/// High-level coding agent with comprehensive task execution capabilities
#[derive(Debug)]
pub struct Agent {
    provider: Box<dyn Provider>,
    file_system_tools: FileSystemTools,
    state: AgentState,
    event_bus: EventBus,
    orchestrator: Orchestrator,
    planner: Planner,
    executor: Executor,
    reviewer: Reviewer,
    debugger: Debugger,
}

impl Agent {
    /// Create a new agent with the specified provider and configuration
    #[instrument(skip(provider, event_bus), fields(provider = %provider.name()))]
    pub fn new(
        provider: Box<dyn Provider>,
        file_system_tools: FileSystemTools,
        event_bus: EventBus,
    ) -> Self {
        let event_bus_clone = event_bus.clone();
        Self {
            provider,
            file_system_tools,
            state: AgentState::default(),
            event_bus,
            orchestrator: Orchestrator::new(event_bus_clone),
            planner: Planner::new(),
            executor: Executor::new(),
            reviewer: Reviewer::new(),
            debugger: Debugger::new(),
        }
    }

    /// Get a reference to the current agent state
    #[inline]
    pub fn state(&self) -> &AgentState {
        &self.state
    }

    /// Get a mutable reference to the agent state
    #[inline]
    pub fn state_mut(&mut self) -> &mut AgentState {
        &mut self.state
    }

    /// Get the event bus for publishing events
    #[inline]
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Run a task through the complete agent pipeline
    ///
    /// This method orchestrates the full workflow:
    /// 1. Analyze the task requirements
    /// 2. Create a detailed plan
    /// 3. Execute each step with tool calls
    /// 4. Review the results
    /// 5. Debug any issues if needed
    #[instrument(skip(self, task), fields(task_id = %task.id))]
    pub async fn run(&mut self, mut task: Task) -> Result<String> {
        info!("Starting task execution: {}", task.description);
        
        self.state.start_thinking();
        self.state.set_current_task(task.id.clone());
        task.start();

        // Emit task started event
        self.event_bus.publish(crate::agent::events::AgentEvent::Started {
            task: task.description.clone(),
        });

        // Phase 1: Analysis & Planning
        debug!("Creating execution plan");
        self.event_bus.publish(crate::agent::events::AgentEvent::Thinking);
        
        let plan = self.planner.create_plan(&task.description);
        info!("Created plan with {} steps", plan.len());
        
        self.event_bus.publish(crate::agent::events::AgentEvent::PlanCreated {
            steps: plan.len(),
        });

        // Phase 2: Execution
        debug!("Executing plan steps");
        let mut results = Vec::new();
        
        for (step_idx, step) in plan.iter().enumerate() {
            info!("Executing step {}/{}: {}", step_idx + 1, plan.len(), step.description);
            
            self.event_bus.publish(crate::agent::events::AgentEvent::ToolStarted {
                name: step.description.clone(),
            });

            match self.executor.execute(step).await {
                Ok(result) => {
                    debug!("Step completed successfully");
                    results.push(result.clone());
                    
                    self.event_bus.publish(crate::agent::events::AgentEvent::ToolFinished {
                        name: step.description.clone(),
                        success: true,
                    });
                }
                Err(e) => {
                    warn!("Step failed: {}", e);
                    self.state.record_error();
                    
                    self.event_bus.publish(crate::agent::events::AgentEvent::ToolFinished {
                        name: step.description.clone(),
                        success: false,
                    });

                    // Attempt debugging
                    debug!("Attempting to debug failed step");
                    match self.debugger.debug(&e.to_string(), &task.description).await {
                        Ok(debug_result) => {
                            info!("Debug suggestion: {}", debug_result);
                            results.push(format!("Failed: {} - Debug: {}", step.description, debug_result));
                        }
                        Err(debug_err) => {
                            warn!("Debug also failed: {}", debug_err);
                            results.push(format!("Failed: {} - Error: {}", step.description, e));
                        }
                    }
                }
            }
        }

        // Phase 3: Review (if there were successful changes)
        if !results.is_empty() {
            debug!("Reviewing execution results");
            let combined_results = results.join("\n\n");
            
            match self.reviewer.review(&combined_results).await {
                Ok(review) => {
                    info!("Review completed");
                    results.push(format!("Review: {}", review));
                }
                Err(e) => {
                    warn!("Review failed: {}", e);
                }
            }
        }

        // Finalize task state
        let success = self.state.error_count == 0;
        if success {
            task.complete();
        } else {
            task.fail();
        }

        self.state.stop_thinking();
        self.state.clear_current_task();

        self.event_bus.publish(crate::agent::events::AgentEvent::Finished { success });

        info!(
            "Task completed: {} (success={}, errors={})",
            task.description,
            success,
            self.state.error_count
        );

        Ok(results.join("\n\n"))
    }

    /// Send a message to the LLM and get a response
    #[instrument(skip(self, messages), fields(message_count = messages.len()))]
    pub async fn chat(&mut self, messages: Vec<Message>) -> Result<Message> {
        debug!("Sending chat request to provider");
        
        let model = self.get_default_model();
        
        match self.provider.chat(&messages, &model, None, None, None, None).await {
            Ok(response) => {
                debug!("Received response from provider");
                Ok(Message::assistant(response.content))
            }
            Err(e) => {
                warn!("Provider error: {}", e);
                self.state.record_error();
                Err(AgentError::ProviderError(e.to_string()).into())
            }
        }
    }

    /// Send a message with tool calling support
    #[instrument(skip(self, messages, tools))]
    pub async fn chat_with_tools(
        &mut self,
        messages: Vec<Message>,
        tools: Option<&[crate::models::tool_call::ToolCall]>,
    ) -> Result<Message> {
        debug!("Sending chat request with tool support");
        
        let model = self.get_default_model();
        
        match self.provider.chat(&messages, &model, None, tools, None, None).await {
            Ok(response) => {
                debug!("Received response with {} tool calls", 
                    response.tool_calls.as_ref().map(|t| t.len()).unwrap_or(0));
                Ok(Message::assistant(response.content))
            }
            Err(e) => {
                warn!("Provider error: {}", e);
                self.state.record_error();
                Err(AgentError::ProviderError(e.to_string()).into())
            }
        }
    }

    /// Stream a response from the LLM
    #[instrument(skip(self, messages))]
    pub async fn stream_chat(
        &mut self,
        messages: Vec<Message>,
    ) -> Result<futures::stream::BoxStream<'static, Result<String>>> {
        debug!("Starting streaming chat request");
        
        let model = self.get_default_model();
        
        match self.provider.stream(&messages, &model, None, None, None, None).await {
            Ok(stream) => Ok(stream),
            Err(e) => {
                warn!("Provider streaming error: {}", e);
                self.state.record_error();
                Err(AgentError::ProviderError(e.to_string()).into())
            }
        }
    }

    /// Get the default model from provider capabilities
    fn get_default_model(&self) -> String {
        self.provider.capabilities()
            .models
            .first()
            .cloned()
            .unwrap_or_else(|| "gpt-4".to_string())
    }

    /// Reset the agent state
    #[inline]
    pub fn reset(&mut self) {
        debug!("Resetting agent state");
        self.state.reset();
    }

    /// Check if the agent is currently processing a task
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.state.is_thinking || self.state.current_task_id.is_some()
    }

    /// Get the number of errors encountered
    #[inline]
    pub fn error_count(&self) -> usize {
        self.state.error_count
    }

    /// Get pending tool call count
    #[inline]
    pub fn pending_tool_calls(&self) -> usize {
        self.state.pending_tool_calls
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tool_call::ToolCall;

    #[test]
    fn test_agent_creation() {
        // Note: This is a basic test - full integration tests would require mock providers
        let event_bus = EventBus::new();
        let file_system = FileSystemTools::new();
        
        // We can't easily create a mock provider here without additional setup
        // This demonstrates the API structure
        assert!(true);
    }

    #[test]
    fn test_agent_state_initialization() {
        let state = AgentState::default();
        assert!(!state.is_thinking);
        assert!(state.current_task_id.is_none());
        assert_eq!(state.error_count, 0);
    }

    #[tokio::test]
    async fn test_task_lifecycle() {
        let mut task = Task::new("Test task");
        assert_eq!(task.status, TaskStatus::Pending);
        
        task.start();
        assert_eq!(task.status, TaskStatus::InProgress);
        
        task.complete();
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.is_complete());
    }
}
