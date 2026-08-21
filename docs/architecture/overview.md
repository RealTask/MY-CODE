# Architecture Overview

MY CODE is an AI-powered terminal coding agent. The crate is organized into layers:

- **CLI** (`src/cli`) — argument parsing, subcommands, and shell completions
- **App** (`src/app`) — process lifecycle, runtime state, and command dispatch
- **Agent** (`src/agent`) — planner, executor, reviewer, and debugger loop
- **Providers** (`src/providers`) — LLM backends (OpenAI-compatible, Anthropic, Google, null)
- **Tools** (`src/tools`) — filesystem, terminal, search, git, and code analysis
- **Workspace / Context / Index** — project detection and prompt budgeting
- **Config / Sessions / Database** — user settings and persistence
- **Sandbox / Security** — permission policy and secret detection

The binary entry point is `src/main.rs`. Library consumers can construct an
`Application` and call `run` with a `Command`.
