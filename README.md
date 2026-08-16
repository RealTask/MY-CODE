# MY CODE

**AI-powered terminal coding agent for professional developers.**

![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)

MY CODE is a serious developer tool that allows you to enter a project and say things like:

- "Build authentication."
- "Fix this bug."
- "Explain this repository."
- "Add dark mode."
- "Refactor this API."
- "Run the tests and fix failures."
- "Review my changes."
- "Create a complete React application."

MY CODE inspects your project, understands relevant context, plans the work, makes changes, runs tools, verifies the result, and explains what happened.

![MY CODE Demo](web/preview.png)

## Features

- 🤖 **Intelligent Agent** - Plans, executes, and reviews code changes
- 🔍 **Workspace Understanding** - Detects languages, frameworks, and project structure
- 📁 **Smart Context** - Selects relevant files and symbols automatically
- 🛠️ **Powerful Tools** - Filesystem, terminal, search, git, and code analysis
- 🔒 **Security First** - Permission system, sandboxing, and approval workflows
- 💾 **Sessions & Memory** - Resume work, track history, learn from context
- 🎨 **Beautiful TUI** - Polished terminal interface with streaming output
- 🔌 **Extensible** - Plugin system for custom tools and providers
- 🌐 **Multi-Provider** - OpenAI, Anthropic, Google, and compatible APIs

## Quick Start

```bash
# Install from Cargo
cargo install my-code

# Or build from source
git clone https://github.com/RealTask/MY-CODE.git
cd MY-CODE
cargo build --release

# Initialize in your project
my-code init

# Start interactive session
my-code chat

# Run a task
my-code plan "add user authentication"

# Review changes
my-code review
```

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
- [Configuration](#configuration)
- [Providers](#providers)
- [Documentation](#documentation)
- [Architecture](#architecture)
- [Contributing](#contributing)
- [License](#license)

## Installation

### System Requirements

- **OS**: Linux, macOS, or Windows 10+
- **Rust**: 1.75 or later
- **Git**: Required for version control features
- **ripgrep** (optional): For enhanced search

### From Cargo (Recommended)

```bash
cargo install my-code
```

### From Source

```bash
git clone https://github.com/RealTask/MY-CODE.git
cd MY-CODE
cargo build --release
cargo install --path .
```

### Pre-built Binaries

Download from [releases](https://github.com/RealTask/MY-CODE/releases).

See full installation guide in [docs/getting-started/installation.md](docs/getting-started/installation.md).

## Usage

### Interactive Mode

```bash
my-code chat
```

This starts an interactive session where you can:
- Ask questions about your codebase
- Request feature implementations
- Debug issues
- Review changes

### Command Examples

```bash
# Plan a feature
my-code plan "add JWT authentication"

# Build a feature
my-code build "create REST API endpoints"

# Review changes
my-code review

# Debug an issue
my-code debug "tests failing in auth module"

# Explain code
my-code explain src/main.rs

# Run tests and fix
my-code test

# Undo changes
my-code undo
```

## Commands

| Command | Description |
|---------|-------------|
| `chat` | Interactive coding session |
| `plan` | Create implementation plan |
| `build` | Implement features |
| `review` | Review code changes |
| `debug` | Diagnose and fix issues |
| `test` | Run tests and fix failures |
| `explain` | Explain code or concepts |
| `diff` | View and understand changes |
| `undo` | Revert recent changes |
| `init` | Initialize MY CODE in project |
| `doctor` | Diagnose configuration issues |
| `sessions` | Manage coding sessions |
| `config` | Manage configuration |

See full command reference in [docs/commands/overview.md](docs/commands/overview.md).

## Configuration

MY CODE supports multiple configuration sources:

```toml
# ~/.config/my-code/config.toml

[provider]
default = "openai"

[model]
default = "gpt-4o"

[context]
max_tokens = 128000

[permissions]
approval_mode = "confirm"
```

See full configuration guide in [docs/configuration.md](docs/configuration.md).

## Providers

MY CODE supports multiple AI providers:

| Provider | Models |
|----------|--------|
| OpenAI | GPT-4o, GPT-4 Turbo, GPT-3.5 |
| Anthropic | Claude Sonnet, Claude Opus |
| Google | Gemini 2.0 Flash, Gemini 2.0 Pro |
| Custom | Any OpenAI-compatible API |
| Local | Ollama, LM Studio |

Configure your provider:

```bash
export MY_CODE_PROVIDER="openai"
export MY_CODE_API_KEY="sk-..."
```

See provider setup guide in [docs/providers/setup.md](docs/providers/setup.md).

## Documentation

| Guide | Description |
|-------|-------------|
| [Installation](docs/getting-started/installation.md) | Install and setup |
| [Configuration](docs/configuration.md) | Customize behavior |
| [Providers](docs/providers/setup.md) | Configure AI providers |
| [Commands](docs/commands/overview.md) | Command reference |
| [Architecture](docs/architecture/overview.md) | System design |

## Architecture

MY CODE uses a layered architecture:

```
┌─────────────────┐
│      CLI        │
├─────────────────┤
│   Application   │
├─────────────────┤
│     Agent       │
├─────────────────┤
│    Providers    │
├─────────────────┤
│     Tools       │
├─────────────────┤
│   Workspace     │
├─────────────────┤
│    Context      │
└─────────────────┘
```

See full architecture documentation in [docs/architecture/](docs/architecture/).

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
git clone https://github.com/RealTask/MY-CODE.git
cd MY-CODE
cargo build
cargo test
```

### Running Tests

```bash
cargo test          # Run all tests
cargo clippy        # Lint code
cargo fmt           # Format code
```

## Security

MY CODE is designed with security in mind:

- Permission system with approval workflows
- Protection against dangerous operations
- Secret detection and redaction
- Audit trail for important actions
- Sandboxed execution where possible

See [SECURITY.md](SECURITY.md) for details.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Authors

- **RealTask** - [GitHub](https://github.com/RealTask)

## Acknowledgments

Inspired by modern open-source coding agents and developer tools including Aider, OpenCode, and others in the community.

---

<p align="center">
  <strong>Built with ❤️ using Rust</strong>
</p>
