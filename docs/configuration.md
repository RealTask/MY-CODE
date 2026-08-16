# Configuration

MY CODE supports flexible configuration through multiple sources with a clear priority order.

## Configuration Priority

Configuration is loaded in the following order (highest to lowest priority):

1. **CLI Arguments** - Command-line flags override everything
2. **Environment Variables** - System environment settings
3. **Project Configuration** - `.my-code/config.toml` in your project
4. **User Configuration** - `~/.config/my-code/config.toml`
5. **Defaults** - Built-in default values

## Configuration Files

### User Configuration

Located at `~/.config/my-code/config.toml` (Linux/macOS) or `%APPDATA%\my-code\config.toml` (Windows).

```toml
# ~/.config/my-code/config.toml

# Default provider
[provider]
default = "openai"
api_key = "your-api-key"  # Or use MY_CODE_API_KEY env var

# Default model
[model]
default = "gpt-4o"

# Context limits
[context]
max_tokens = 128000
budget_percentage = 80

# Permission settings
[permissions]
approval_mode = "confirm"  # safe | confirm | dangerous | blocked

# Theme
[tui]
theme = "dark"  # dark | light | auto

# Logging
[logging]
level = "info"  # error | warn | info | debug | trace
file = "~/.local/state/my-code/my-code.log"
```

### Project Configuration

Located at `.my-code/config.toml` in your project root.

```toml
# .my-code/config.toml

# Project-specific provider/model
[provider]
default = "anthropic"

[model]
default = "claude-sonnet-4-5-20250929"

# Project-specific instructions
[instructions]
architecture = "This is a Next.js application using TypeScript and Tailwind CSS."
conventions = "Use functional components with hooks. Follow ESLint rules strictly."
testing = "Run tests with: npm test"

# Custom context rules
[context]
include = ["src/**/*.ts", "src/**/*.tsx"]
exclude = ["node_modules", ".next", "dist"]

# Tool-specific settings
[tools.filesystem]
backup_before_edit = true

[tools.terminal]
timeout_seconds = 60
```

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `MY_CODE_WORKSPACE` | Default workspace path | `/path/to/project` |
| `MY_CODE_PROVIDER` | Default AI provider | `openai`, `anthropic`, `google` |
| `MY_CODE_MODEL` | Default model name | `gpt-4o`, `claude-sonnet-4-5-20250929` |
| `MY_CODE_API_KEY` | API key for providers | `sk-...` |
| `MY_CODE_SESSION` | Default session ID | `session-uuid` |
| `MY_CODE_APPROVAL_MODE` | Permission level | `safe`, `confirm`, `dangerous` |
| `MY_CODE_VERBOSE` | Enable verbose output | `true`, `false` |
| `MY_CODE_NO_COLOR` | Disable colored output | `true`, `false` |
| `MY_CODE_CONFIG` | Custom config file path | `/path/to/config.toml` |

## Provider Configuration

### OpenAI

```toml
[provider.openai]
api_key = "sk-..."  # Or use MY_CODE_OPENAI_API_KEY
base_url = "https://api.openai.com/v1"
models = ["gpt-4o", "gpt-4-turbo", "gpt-3.5-turbo"]
```

### Anthropic

```toml
[provider.anthropic]
api_key = "sk-ant-..."  # Or use MY_CODE_ANTHROPIC_API_KEY
models = ["claude-sonnet-4-5-20250929", "claude-opus-4-5-20250929"]
```

### Google

```toml
[provider.google]
api_key = "..."  # Or use MY_CODE_GOOGLE_API_KEY
models = ["gemini-2.0-flash", "gemini-2.0-pro"]
```

### OpenAI-Compatible APIs

```toml
[provider.custom]
name = "Custom Provider"
api_key = "..."
base_url = "https://your-api.com/v1"
models = ["custom-model-1", "custom-model-2"]
```

## Context Configuration

Control how MY CODE selects and manages context:

```toml
[context]
# Maximum tokens to send to the model
max_tokens = 128000

# Percentage of max_tokens to use for context
budget_percentage = 80

# Always include these files
always_include = [".my-code/instructions.md", "README.md"]

# Never include these patterns
never_include = ["**/*.lock", "**/minified/**", "**/vendor/**"]

# File size limits (in bytes)
max_file_size = 1048576  # 1MB

# Maximum number of files in context
max_files = 100
```

## Permission Levels

| Level | Description | Examples |
|-------|-------------|----------|
| `safe` | No confirmation needed | Reading files, running tests |
| `confirm` | Confirm before executing | Writing files, installing packages |
| `dangerous` | Explicit confirmation required | Deleting files, mass modifications |
| `blocked` | Never allowed | Accessing system directories |

```toml
[permissions]
# Set default approval mode
approval_mode = "confirm"

# Override specific actions
filesystem.write = "confirm"
filesystem.delete = "dangerous"
terminal.execute = "confirm"
network.access = "blocked"
```

## TUI Configuration

```toml
[tui]
# Color theme
theme = "dark"

# Show/hide components
show_sidebar = true
show_status_bar = true
show_line_numbers = true

# Keybindings
[keybindings]
quit = ["Ctrl-c", "q"]
submit = ["Enter"]
cancel = ["Escape"]
scroll_up = ["Ctrl-u", "PageUp"]
scroll_down = ["Ctrl-d", "PageDown"]
```

## Logging Configuration

```toml
[logging]
# Log level: error, warn, info, debug, trace
level = "info"

# Log file location
file = "~/.local/state/my-code/my-code.log"

# Log format: json, pretty
format = "pretty"

# Maximum log file size (in MB)
max_size_mb = 10

# Number of rotated log files to keep
keep_files = 5
```

## Validation

Check your configuration:

```bash
my-code doctor
```

This command validates:
- Configuration file syntax
- Provider credentials
- Required tools availability
- Workspace permissions
- Network connectivity

## Examples

See example configurations in the `config/` directory:
- `config/default.toml` - Default configuration
- `config/example.toml` - Example with all options

## Troubleshooting

### Configuration Not Loading

1. Check file location and syntax
2. Verify file permissions
3. Run `my-code doctor` to diagnose issues

### Provider Authentication Failing

1. Ensure API key is set correctly
2. Check environment variables take precedence
3. Verify network connectivity

### Context Too Large

1. Reduce `max_tokens` in configuration
2. Add more patterns to `never_include`
3. Use specific file paths instead of wildcards

## Next Steps

- [Provider Setup](providers/openai.md)
- [Commands](../commands/chat.md)
- [Plugins](../plugins/overview.md)
