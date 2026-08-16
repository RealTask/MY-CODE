# Commands

MY CODE provides powerful commands for various development workflows.

## Interactive Mode

### Chat

Start an interactive coding session:

```bash
my-code chat
```

Or simply:

```bash
my-code
```

**Features:**
- Real-time conversation with AI
- Streaming responses
- Tool execution with approval prompts
- File editing with previews
- Command execution
- Session persistence

**Example Session:**
```
$ my-code chat

┌─────────────────────────────────────────────┐
│ MY CODE                         gpt-4o | ✓  │
├─────────────────────────────────────────────┤
│                                             │
│ > How can I help you today?                 │
│                                             │
│                                             │
├─────────────────────────────────────────────┤
│ Ready                                       │
├─────────────────────────────────────────────┤
│ > Build a login form with validation        │
└─────────────────────────────────────────────┘
```

## Planning

### Plan

Create a detailed implementation plan without making changes:

```bash
my-code plan "add user authentication with JWT"
```

**Output:**
- Analysis of current codebase
- Step-by-step implementation plan
- Files to be created/modified
- Dependencies and considerations
- Estimated complexity

**Options:**
```bash
my-code plan "feature" --output plan.md  # Save to file
my-code plan "feature" --no-context      # Skip context analysis
```

## Building & Implementation

### Build

Implement features or create new code:

```bash
my-code build "create a REST API with Express"
```

**What it does:**
1. Analyzes the request
2. Selects relevant context
3. Plans the implementation
4. Makes code changes
5. Runs verification steps
6. Reports results

**Options:**
```bash
my-code build "feature" --dry-run      # Preview changes only
my-code build "feature" --no-test      # Skip running tests
my-code build "feature" --branch feat  # Create feature branch
```

## Review & Analysis

### Review

Review code changes or pull requests:

```bash
my-code review
```

Reviews staged changes or current branch.

```bash
my-code review --branch main
my-code review --commit abc123
```

**Review includes:**
- Code quality assessment
- Potential bugs
- Security concerns
- Performance considerations
- Style consistency
- Suggestions for improvement

### Explain

Understand code, files, or concepts:

```bash
my-code explain src/auth/jwt.rs
my-code explain "how does the middleware chain work?"
my-code explain package.json
```

**Explains:**
- Code functionality
- Architecture patterns
- Dependencies
- Data flow
- Design decisions

### Debug

Diagnose and fix issues:

```bash
my-code debug "tests are failing in auth module"
my-code debug --error "cannot connect to database"
```

**Debug process:**
1. Analyzes error messages
2. Examines relevant code
3. Identifies root cause
4. Suggests fixes
5. Applies corrections
6. Verifies solution

## Testing

### Test

Run tests and fix failures:

```bash
my-code test
```

**Features:**
- Detects test framework automatically
- Runs appropriate test command
- Analyzes failures
- Attempts automatic fixes
- Re-runs tests to verify

**Options:**
```bash
my-code test --watch          # Watch mode
my-code test --coverage       # Include coverage
my-code test --file auth.rs   # Test specific file
```

## Git Operations

### Diff

View and understand changes:

```bash
my-code diff
my-code diff --staged
my-code diff HEAD~1
```

**Shows:**
- Unified diff format
- Affected functions
- Change summary
- Risk assessment

### Undo

Revert recent changes:

```bash
my-code undo              # Undo last change
my-code undo --list       # List checkpoints
my-code undo --id abc123  # Restore specific checkpoint
```

## Project Management

### Init

Initialize MY CODE in a project:

```bash
my-code init
```

Creates:
- `.my-code/config.toml` - Project configuration
- `.my-code/instructions.md` - Project-specific instructions
- `.gitignore` entries

### Doctor

Diagnose configuration and environment issues:

```bash
my-code doctor
```

**Checks:**
- Configuration validity
- Provider credentials
- Required tools (git, ripgrep)
- Workspace permissions
- Network connectivity
- Rate limits

**Output:**
```
MY CODE Doctor

Environment:
✓ Rust 1.75.0
✓ Git 2.40.0
✓ ripgrep 14.0.0

Configuration:
✓ Config file loaded
✓ Provider: openai
✓ Model: gpt-4o
✓ API key configured

Workspace:
✓ Git repository detected
✓ Read/write permissions OK

Network:
✓ API endpoint reachable
✓ Rate limits OK

All systems operational!
```

## Sessions

### Sessions

Manage coding sessions:

```bash
my-code sessions              # List all sessions
my-code sessions --new        # Create new session
my-code sessions --load abc   # Load specific session
my-code sessions --delete xyz # Delete session
```

## Configuration

### Config

Manage configuration:

```bash
my-code config show           # Show current config
my-code config edit           # Open config in editor
my-code config provider       # Change provider
my-code config model          # Change model
```

## Advanced Usage

### Headless Mode

Run non-interactively for CI/CD:

```bash
my-code run "fix all failing tests" --non-interactive
my-code run "add logging" --json --output result.json
```

**Options:**
- `--non-interactive`: No prompts, use defaults
- `--json`: Machine-readable output
- `--quiet`: Minimal output
- `--verbose`: Detailed logging

### Custom Workspace

```bash
my-code chat --workspace /path/to/project
```

### Model Selection

```bash
my-code chat --model claude-sonnet-4-5-20250929
my-code build "feature" --provider anthropic
```

### Approval Modes

```bash
my-code chat --approval safe         # Auto-approve safe actions
my-code chat --approval confirm      # Confirm before changes
my-code chat --approval dangerous    # Explicit confirmation
```

## Command Shortcuts

| Full Command | Shortcut |
|--------------|----------|
| `my-code chat` | `my-code` |
| `my-code plan` | `my-code p` |
| `my-code build` | `my-code b` |
| `my-code review` | `my-code r` |
| `my-code debug` | `my-code d` |
| `my-code test` | `my-code t` |
| `my-code explain` | `my-code e` |
| `my-code diff` | `my-code di` |
| `my-code undo` | `my-code u` |

## Examples

### Quick Start Workflow

```bash
# Enter your project
cd my-app

# Initialize
my-code init

# Start chatting
my-code

# In the chat:
# "Add user registration with email verification"
```

### Feature Development

```bash
# Plan the feature
my-code plan "add password reset flow"

# Implement
my-code build "implement password reset"

# Run tests
my-code test

# Review changes
my-code review

# Commit
git commit -m "feat: add password reset"
```

### Bug Fix Workflow

```bash
# Debug the issue
my-code debug "login fails with invalid token error"

# Verify fix
my-code test

# Review what changed
my-code diff

# Undo if needed
my-code undo
```

### Code Review

```bash
# Review current changes
my-code review

# Get explanation
my-code explain src/complex-module.rs

# Ask questions
my-code "why is this pattern used here?"
```

## Best Practices

1. **Be Specific**: Clear requests get better results
2. **Review Changes**: Always review AI-generated code
3. **Use Planning**: For complex features, start with `plan`
4. **Test Frequently**: Run `test` after changes
5. **Save Sessions**: Important conversations are saved automatically
6. **Use Checkpoints**: `undo` is your friend

## Next Steps

- [Chat Command](commands/chat.md) - Detailed chat usage
- [Provider Setup](providers/setup.md) - Configure AI providers
- [Configuration](configuration.md) - Customize behavior
