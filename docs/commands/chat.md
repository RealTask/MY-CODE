# Chat command

Start an interactive session in the current workspace:

```bash
my-code chat
my-code chat "Build authentication"
```

Options inherited from the root command:

- `--model` / `-m` — override the configured model
- `--provider` / `-p` — override the configured provider
- `--config` / `-c` — path to a TOML config file
- `--approval-mode` — `auto`, `confirm`, or `always`

If no subcommand is given, `my-code` defaults to `chat`.
