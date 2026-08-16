# Installation

Get MY CODE running on your system in minutes.

## System Requirements

- **Operating System**: Linux, macOS, or Windows 10+
- **Rust**: 1.75 or later (for building from source)
- **Git**: Required for version control features
- **ripgrep** (optional): For enhanced search capabilities

## Installation Methods

### Using Cargo (Recommended)

```bash
cargo install my-code
```

This will download and compile the latest stable release from crates.io.

### From Source

```bash
# Clone the repository
git clone https://github.com/RealTask/MY-CODE.git
cd MY-CODE

# Build and install
cargo install --path .
```

### Pre-built Binaries

Download pre-built binaries from our [releases page](https://github.com/RealTask/MY-CODE/releases).

#### Linux
```bash
# Download and extract
wget https://github.com/RealTask/MY-CODE/releases/latest/download/my-code-linux-x86_64.tar.gz
tar -xzf my-code-linux-x86_64.tar.gz
sudo mv my-code /usr/local/bin/
```

#### macOS
```bash
# Download and extract
curl -LO https://github.com/RealTask/MY-CODE/releases/latest/download/my-code-macos-x86_64.tar.gz
tar -xzf my-code-macos-x86_64.tar.gz
sudo mv my-code /usr/local/bin/
```

#### Windows
```powershell
# Download using PowerShell
Invoke-WebRequest -Uri "https://github.com/RealTask/MY-CODE/releases/latest/download/my-code-windows-x86_64.zip" -OutFile "my-code.zip"
Expand-Archive my-code.zip -DestinationPath $env:USERPROFILE\my-code
# Add to PATH manually or move to a directory in PATH
```

## Verification

Verify the installation:

```bash
my-code --version
```

You should see output like:
```
my-code 0.1.0
```

## First Run

Initialize MY CODE in your project:

```bash
cd your-project
my-code init
```

This creates a `.my-code/config.toml` file with default settings.

## Shell Completions

Generate shell completions for better CLI experience:

```bash
# Bash
my-code completions bash > ~/.bash_completion.d/my-code

# Zsh
my-code completions zsh > ~/.zsh/completion/_my-code

# Fish
my-code completions fish > ~/.config/fish/completions/my-code.fish

# PowerShell
my-code completions powershell > $PROFILE
```

## Updating

Update to the latest version:

```bash
cargo install --force my-code
```

Or if installed from source:

```bash
git pull origin main
cargo install --force --path .
```

## Uninstallation

```bash
# If installed via cargo
cargo uninstall my-code

# Remove configuration
rm -rf ~/.config/my-code
rm -rf ~/.my-code
```

## Next Steps

- [Configuration Guide](configuration.md)
- [First Commands](../commands/chat.md)
- [Provider Setup](providers.md)
