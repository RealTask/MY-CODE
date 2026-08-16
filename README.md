# MY CODE

**AI-powered terminal coding agent for professional developers.**

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
# Install
cargo install my-code

# Initialize in your project
my-code init

# Start interactive session
my-code chat

# Run a task
my-code plan "add user authentication"

# Review changes
my-code review
```

## Documentation

- [Architecture](docs/architecture/overview.md)
- [Commands](docs/commands/chat.md)
- [Configuration](docs/configuration.md)
- [Plugins](docs/plugins.md)

## License

MIT OR Apache-2.0

## Authors

RealTask
