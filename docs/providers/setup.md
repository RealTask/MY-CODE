# Provider Setup

MY CODE supports multiple AI providers. Configure your preferred provider to get started.

## Supported Providers

| Provider | Models | Features |
|----------|--------|----------|
| OpenAI | GPT-4o, GPT-4 Turbo, GPT-3.5 | Streaming, Tool Calling, Vision |
| Anthropic | Claude Sonnet, Claude Opus | Streaming, Tool Calling, Large Context |
| Google | Gemini 2.0 Flash, Gemini 2.0 Pro | Streaming, Tool Calling, Multimodal |
| OpenAI-Compatible | Custom models | Any OpenAI-compatible API |
| Local | Ollama, LM Studio | Self-hosted, Private |

## OpenAI

### Get API Key

1. Visit [platform.openai.com](https://platform.openai.com)
2. Sign in or create an account
3. Go to API Keys section
4. Create a new API key

### Configuration

```toml
# ~/.config/my-code/config.toml

[provider]
default = "openai"

[provider.openai]
api_key = "sk-..."  # Or use MY_CODE_OPENAI_API_KEY env var
base_url = "https://api.openai.com/v1"

[model]
default = "gpt-4o"
```

### Environment Variable

```bash
export MY_CODE_OPENAI_API_KEY="sk-..."
export MY_CODE_PROVIDER="openai"
export MY_CODE_MODEL="gpt-4o"
```

### Available Models

- `gpt-4o` - Latest multimodal model (recommended)
- `gpt-4-turbo` - Fast GPT-4 variant
- `gpt-3.5-turbo` - Cost-effective option

## Anthropic

### Get API Key

1. Visit [console.anthropic.com](https://console.anthropic.com)
2. Sign in or create an account
3. Go to API Keys section
4. Create a new API key

### Configuration

```toml
# ~/.config/my-code/config.toml

[provider]
default = "anthropic"

[provider.anthropic]
api_key = "sk-ant-..."  # Or use MY_CODE_ANTHROPIC_API_KEY env var

[model]
default = "claude-sonnet-4-5-20250929"
```

### Environment Variable

```bash
export MY_CODE_ANTHROPIC_API_KEY="sk-ant-..."
export MY_CODE_PROVIDER="anthropic"
export MY_CODE_MODEL="claude-sonnet-4-5-20250929"
```

### Available Models

- `claude-sonnet-4-5-20250929` - Best balance of speed and intelligence (recommended)
- `claude-opus-4-5-20250929` - Most powerful for complex tasks
- `claude-haiku-3-5-20241022` - Fast and cost-effective

## Google

### Get API Key

1. Visit [Google AI Studio](https://makersuite.google.com)
2. Sign in with your Google account
3. Create an API key

### Configuration

```toml
# ~/.config/my-code/config.toml

[provider]
default = "google"

[provider.google]
api_key = "..."  # Or use MY_CODE_GOOGLE_API_KEY env var

[model]
default = "gemini-2.0-flash"
```

### Environment Variable

```bash
export MY_CODE_GOOGLE_API_KEY="..."
export MY_CODE_PROVIDER="google"
export MY_CODE_MODEL="gemini-2.0-flash"
```

### Available Models

- `gemini-2.0-flash` - Fast and capable (recommended)
- `gemini-2.0-pro` - Most powerful Gemini model

## OpenAI-Compatible APIs

Many providers offer OpenAI-compatible endpoints.

### Configuration

```toml
# ~/.config/my-code/config.toml

[provider]
default = "custom"

[provider.custom]
name = "My Custom Provider"
api_key = "your-api-key"
base_url = "https://api.example.com/v1"
models = ["model-1", "model-2"]

[model]
default = "model-1"
```

### Popular Compatible Providers

- **Azure OpenAI**: `https://{resource}.openai.azure.com/openai/deployments/{deployment}`
- **Groq**: `https://api.groq.com/openai/v1`
- **Together AI**: `https://api.together.xyz/v1`
- **Fireworks**: `https://api.fireworks.ai/inference/v1`
- **Ollama** (local): `http://localhost:11434/v1`

## Local Models

### Ollama

1. Install Ollama from [ollama.ai](https://ollama.ai)
2. Pull a model: `ollama pull llama2`
3. Configure MY CODE:

```toml
[provider]
default = "ollama"

[provider.ollama]
base_url = "http://localhost:11434/v1"
api_key = "ollama"  # Not required but some versions need it

[model]
default = "llama2"
```

### LM Studio

1. Install LM Studio from [lmstudio.ai](https://lmstudio.ai)
2. Download a model
3. Start the local server
4. Configure:

```toml
[provider]
default = "lmstudio"

[provider.lmstudio]
base_url = "http://localhost:1234/v1"

[model]
default = "local-model"
```

## Model Selection

### By Use Case

| Task | Recommended Model |
|------|-------------------|
| General coding | `gpt-4o`, `claude-sonnet-4-5-20250929` |
| Complex refactoring | `claude-opus-4-5-20250929`, `gpt-4o` |
| Quick tasks | `gpt-3.5-turbo`, `claude-haiku-3-5-20241022` |
| Large codebases | `claude-sonnet-4-5-20250929` (200K context) |
| Privacy-sensitive | Local models (Ollama, LM Studio) |

### Switching Models

Temporarily override the default model:

```bash
my-code chat --model gpt-4-turbo
my-code plan "add authentication" --model claude-opus-4-5-20250929
```

## Testing Your Setup

Run the doctor command to verify your configuration:

```bash
my-code doctor
```

Expected output:
```
✓ Configuration loaded
✓ Provider: openai
✓ Model: gpt-4o
✓ API key configured
✓ Network connectivity OK
✓ Rate limits: OK
```

## Troubleshooting

### Authentication Errors

1. Verify API key is correct
2. Check for extra whitespace
3. Ensure environment variable is set: `echo $MY_CODE_API_KEY`
4. Try regenerating the API key

### Rate Limits

If you hit rate limits:

1. Check your provider dashboard for limits
2. Reduce request frequency
3. Upgrade your plan if needed
4. Add retry configuration:

```toml
[provider.openai]
retry_attempts = 3
retry_delay_ms = 1000
```

### Model Not Found

1. Verify model name is correct
2. Check model availability in your region
3. Ensure your API key has access to the model
4. Some models require specific access tiers

### Connection Timeouts

1. Check network connectivity
2. Verify firewall settings
3. Try a different base URL if using custom provider
4. Increase timeout settings:

```toml
[provider.openai]
timeout_seconds = 60
```

## Security Best Practices

1. **Never commit API keys** to version control
2. **Use environment variables** for sensitive data
3. **Rotate keys regularly** through your provider dashboard
4. **Set usage limits** in your provider account
5. **Monitor usage** for unexpected activity

## Next Steps

- [Commands](../commands/chat.md)
- [Context Configuration](configuration.md#context-configuration)
- [Custom Providers](../examples/custom-provider/)
