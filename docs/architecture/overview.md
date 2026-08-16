# Architecture Overview

This document describes the high-level architecture of MY CODE.

## Design Principles

MY CODE is built following these core principles:

1. **Layered Architecture** - Clear separation of concerns with dependencies flowing toward stable abstractions
2. **Trait-Based Extensibility** - Use traits where they provide real extensibility without over-engineering
3. **Async-First** - Leverage async Rust for I/O-bound operations
4. **Error Resilience** - Structured error handling with recovery suggestions
5. **Security by Default** - Permission system and sandboxing built into the core

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│  (args, commands, parser, completions)                       │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                      Application Layer                       │
│  (application coordinator, lifecycle, global state)          │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                        Agent Layer                           │
│  (agent, orchestrator, planner, executor, reviewer)          │
└─────────────────────────────────────────────────────────────┘
                              ↓
        ┌───────────────────┴───────────────────┐
        ↓                                       ↓
┌──────────────────┐                  ┌──────────────────┐
│   Model Layer    │                  │    Tool Layer    │
│ (messages, tool  │                  │ (registry,       │
│  calls, results) │                  │  dispatcher)     │
└──────────────────┘                  └──────────────────┘
        ↓                                       ↓
┌──────────────────┐                  ┌──────────────────┐
│   Providers      │                  │  Tool Categories │
│ (OpenAI,         │                  │ - Filesystem     │
│  Anthropic,      │                  │ - Terminal       │
│  Google, etc.)   │                  │ - Search         │
└──────────────────┘                  │ - Git            │
                                      │ - Code           │
                                      └──────────────────┘
                                                ↓
┌─────────────────────────────────────────────────────────────┐
│                    Workspace Layer                           │
│  (detector, scanner, file tree, languages, frameworks)       │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                     Context Layer                            │
│  (engine, collector, selector, ranking, budget, cache)       │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   Supporting Systems                         │
│  - Index (code symbols, dependencies)                        │
│  - Parser (Tree-sitter integration)                          │
│  - Edit Engine (patch application, validation)               │
│  - Diff Engine (unified diff, rendering)                     │
│  - Git Integration (repository operations)                   │
│  - Checkpoint System (undo/restore)                          │
│  - Sandbox (permission enforcement)                          │
│  - Security (secrets, audit)                                 │
│  - Sessions (conversation history)                           │
│  - Memory (project knowledge)                                │
│  - Database (SQLite persistence)                             │
│  - Plugins (extension system)                                │
│  - Events (internal event bus)                               │
│  - TUI (terminal interface)                                  │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### CLI Layer

The CLI layer handles:
- Command-line argument parsing with `clap`
- Command routing and execution
- Shell completion generation
- Exit code management

**Key Files:**
- `src/cli/args.rs` - Global argument definitions
- `src/cli/commands.rs` - Command enum and handlers
- `src/cli/parser.rs` - Argument parsing logic
- `src/cli/completions.rs` - Shell completion generation

### Application Layer

Coordinates the entire application:
- Service initialization and registration
- Configuration loading
- Lifecycle management (startup/shutdown)
- Global state management

**Key Files:**
- `src/app/application.rs` - Central coordinator
- `src/app/lifecycle.rs` - Startup/shutdown hooks
- `src/app/state.rs` - Global runtime state

### Agent Layer

The intelligent coding agent that:
- Receives user tasks
- Analyzes context and plans work
- Coordinates model calls and tool execution
- Tracks progress and state
- Reviews and validates results

**Key Files:**
- `src/agent/agent.rs` - High-level agent interface
- `src/agent/orchestrator.rs` - Agent loop control
- `src/agent/planner.rs` - Implementation planning
- `src/agent/executor.rs` - Action execution
- `src/agent/reviewer.rs` - Change review
- `src/agent/debugger.rs` - Error investigation

### Model Layer

Abstracts AI model interactions:
- Message formatting (system, user, assistant, tool)
- Tool call parsing and result handling
- Usage tracking (tokens, cost, latency)
- Streaming response handling

**Key Files:**
- `src/models/message.rs` - Message types
- `src/models/tool_call.rs` - Tool call representation
- `src/models/tool_result.rs` - Tool execution results
- `src/models/usage.rs` - Token and cost tracking

### Provider Layer

Implements adapters for different AI providers:
- OpenAI (GPT-4o, GPT-4 Turbo, GPT-3.5)
- Anthropic (Claude Sonnet, Claude Opus)
- Google (Gemini models)
- OpenAI-compatible APIs
- Local models (Ollama, LM Studio)

**Key Files:**
- `src/providers/provider.rs` - Provider trait
- `src/providers/registry.rs` - Provider registry
- `src/providers/manager.rs` - Provider selection and fallback
- `src/providers/openai.rs` - OpenAI adapter
- `src/providers/anthropic.rs` - Anthropic adapter
- `src/providers/streaming.rs` - Streaming support

### Tool Layer

Provides capabilities to the agent:
- **Filesystem**: read, write, edit, delete, move, metadata
- **Terminal**: execute commands, manage processes
- **Search**: filename, text, symbol, reference search
- **Git**: status, diff, history, branches, commits
- **Code**: parsing, symbols, diagnostics

**Key Files:**
- `src/tools/tool.rs` - Tool trait definition
- `src/tools/registry.rs` - Tool registration
- `src/tools/dispatcher.rs` - Tool call routing
- `src/tools/permissions.rs` - Permission requirements

### Workspace Layer

Understands project structure:
- Language detection (Rust, Python, JS/TS, etc.)
- Framework detection (React, Next.js, Django, etc.)
- Project scanning and file tree building
- .gitignore awareness

**Key Files:**
- `src/workspace/detector.rs` - Language/framework detection
- `src/workspace/scanner.rs` - Project scanning
- `src/workspace/file_tree.rs` - Directory tree building
- `src/workspace/project.rs` - Project metadata

### Context Layer

Manages what information goes to the model:
- Intelligent file selection based on request
- Symbol and dependency following
- Relevance ranking
- Token budget management
- Context compression for large projects
- Caching of analysis results

**Key Files:**
- `src/context/engine.rs` - Main context coordination
- `src/context/selector.rs` - File selection logic
- `src/context/ranking.rs` - Relevance ranking
- `src/context/budget.rs` - Token budget management
- `src/context/compression.rs` - Context compression

## Supporting Systems

### Index System

Builds and maintains a codebase index:
- File indexing with metadata
- Symbol extraction (functions, classes, structs)
- Dependency graph construction
- Code graph for relationships
- Persistent storage in SQLite

**Key Files:**
- `src/index/indexer.rs` - Indexing coordination
- `src/index/symbol_index.rs` - Symbol tracking
- `src/index/dependency_graph.rs` - Dependencies

### Edit Engine

Reliable code modification:
- Patch generation from model output
- Target validation before application
- Safe patch application
- Verification after changes
- Rollback on failure

**Key Files:**
- `src/edit/editor.rs` - Edit coordination
- `src/edit/patch.rs` - Patch generation
- `src/edit/applier.rs` - Patch application
- `src/edit/validator.rs` - Change validation

### Diff System

Change visualization:
- Unified diff generation
- Syntax-highlighted rendering
- Affected function detection
- Change summary and risk assessment

**Key Files:**
- `src/diff/engine.rs` - Diff generation
- `src/diff/renderer.rs` - Terminal rendering
- `src/diff/summary.rs` - Change summaries

### Git Integration

Version control operations:
- Repository status and diff
- Commit history browsing
- Branch management
- Checkpoint creation for undo

**Key Files:**
- `src/git/repository.rs` - Repository wrapper
- `src/git/checkpoint.rs` - Undo checkpoints
- `src/git/operations.rs` - Git operations

### Checkpoint System

Enables undo functionality:
- Snapshot creation before changes
- Original content preservation
- Restore capability
- Checkpoint listing and management

**Key Files:**
- `src/checkpoints/checkpoint.rs` - Checkpoint data
- `src/checkpoints/manager.rs` - Checkpoint lifecycle
- `src/checkpoints/restore.rs` - Restoration logic

### Sandbox System

Security enforcement:
- Permission levels (SAFE, CONFIRM, DANGEROUS, BLOCKED)
- Command execution policies
- Filesystem access policies
- Network access control
- Resource limits

**Key Files:**
- `src/sandbox/policy.rs` - Permission policies
- `src/sandbox/command_policy.rs` - Command restrictions
- `src/sandbox/filesystem_policy.rs` - File access rules

### Security System

Protects sensitive data:
- Secret detection in files
- API key protection
- Credential redaction
- Audit trail for actions
- Input validation

**Key Files:**
- `src/security/secrets.rs` - Secret detection
- `src/security/audit.rs` - Action logging
- `src/security/validation.rs` - Input validation

### Session System

Conversation management:
- Session creation and restoration
- Message history storage
- Tool call tracking
- File change tracking
- Cross-session continuity

**Key Files:**
- `src/sessions/session.rs` - Session data
- `src/sessions/manager.rs` - Session lifecycle
- `src/sessions/history.rs` - Message history

### Memory System

Knowledge persistence:
- Conversation memory
- Project-specific knowledge
- Developer instructions
- Architecture notes
- Summaries of past work

**Key Files:**
- `src/memory/memory.rs` - Memory coordination
- `src/memory/project.rs` - Project knowledge
- `src/memory/store.rs` - Persistent storage

### Database System

SQLite-backed persistence:
- Session storage
- Message history
- Checkpoint data
- Project memory
- Index metadata

**Key Files:**
- `src/database/database.rs` - Database connection
- `src/database/migrations.rs` - Schema migrations
- `src/database/sessions.rs` - Session tables

### Plugin System

Extensibility framework:
- Plugin discovery and loading
- Manifest validation
- API for plugin developers
- Version compatibility checking

**Key Files:**
- `src/plugins/plugin.rs` - Plugin trait
- `src/plugins/registry.rs` - Plugin registry
- `src/plugins/loader.rs` - Plugin loading

### Event System

Internal communication:
- Event bus for decoupled communication
- Event types for all significant actions
- Subscriber pattern for UI updates
- Async event handling

**Key Files:**
- `src/events/event.rs` - Event types
- `src/events/bus.rs` - Event bus implementation
- `src/events/handlers.rs` - Event handlers

### TUI System

Terminal user interface:
- Ratatui-based rendering
- Chat interface with streaming
- Sidebar for context
- Status bar for progress
- Approval dialogs
- Syntax highlighting
- Diff previews

**Key Files:**
- `src/tui/app.rs` - TUI application
- `src/tui/state.rs` - UI state
- `src/tui/widgets/chat.rs` - Chat widget
- `src/tui/screens/chat.rs` - Chat screen

## Data Flow

### Typical Request Flow

```
User Input (CLI/TUI)
        ↓
Application Layer
        ↓
Agent Orchestrator
        ↓
    ┌───┴───┐
    ↓       ↓
Context  Planner
Engine   
    ↓       ↓
Files   Plan
Selected
    ↓       ↓
    └───┬───┘
        ↓
    Model Call
    (Provider)
        ↓
    Response
    (with tool calls)
        ↓
    Tool Dispatcher
        ↓
    Tool Execution
        ↓
    Result to Model
        ↓
    Final Response
        ↓
    Validation
        ↓
    Checkpoint
        ↓
User Output
```

## Extension Points

MY CODE is designed to be extended:

1. **Custom Providers** - Implement the `Provider` trait
2. **Custom Tools** - Implement the `Tool` trait and register
3. **Custom Commands** - Add to command enum and handler
4. **Plugins** - Create plugin with manifest and implement plugin API
5. **Custom Prompts** - Extend prompt templates system

See `examples/` directory for extension examples.

## Threading Model

- **Main Thread**: CLI parsing, application startup
- **Tokio Runtime**: Async operations (I/O, network, subprocesses)
- **TUI Thread**: Terminal rendering and input (when using TUI)
- **Worker Threads**: Tree-sitter parsing, indexing (thread pool)

## Error Handling Strategy

All errors use structured types with:
- Error category
- Human-readable message
- Underlying cause (when applicable)
- Recovery suggestion

Errors propagate through Result types with `?` operator.
Top-level errors are formatted nicely for users.
Debug mode shows full error chains.

## Testing Strategy

- **Unit Tests**: Individual functions and modules
- **Integration Tests**: Component interactions
- **End-to-End Tests**: Full workflows in temporary directories
- **Snapshot Tests**: TUI rendering, diff output

Run tests with: `cargo test`

## Performance Considerations

- Lazy loading for large projects
- Incremental indexing
- Context caching
- Async I/O throughout
- Minimal allocations in hot paths
- Efficient string handling

## Future Directions

Planned improvements:
- Enhanced code understanding with better Tree-sitter integration
- Multi-file atomic edits
- Improved context ranking with ML
- Additional provider support
- Rich plugin ecosystem
- VS Code extension
- Team collaboration features

---

For detailed component documentation, see individual module docs in this directory.
