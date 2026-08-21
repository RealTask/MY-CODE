# Configuration

MY CODE reads TOML configuration from, in order:

1. `--config <path>` if provided
2. `./my-code.toml`, `./.my-code.toml`, or `./.my-code/config.toml`
3. The user config directory (`~/.config/my-code/config.toml` on Linux)

If none of those files exist, built-in defaults are used.

## Example

```toml
default_provider = "openai"
default_model = "gpt-4o"

[ui]
theme = "dark"
color = true
max_width = 100

[logging]
level = "info"
format = "pretty"

[sandbox]
enabled = true
allow_network = false
allow_shell = true
```

API keys should be supplied via environment variables (`OPENAI_API_KEY`,
`ANTHROPIC_API_KEY`, `GOOGLE_API_KEY`) rather than committed to disk.

Create a project file with:

```bash
my-code init
```
