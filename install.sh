#!/usr/bin/env bash
#
# MY CODE - Advanced Installation Script
# AI-powered terminal coding agent for professional developers
#
# This script provides a comprehensive, production-ready installation experience
# with dependency management, system checks, configuration setup, and more.
#

set -euo pipefail

# =============================================================================
# Configuration & Constants
# =============================================================================

readonly SCRIPT_NAME="MY CODE Installer"
readonly SCRIPT_VERSION="1.0.0"
readonly PROJECT_NAME="my-code"
readonly BINARY_NAME="my-code"
readonly GITHUB_REPO="RealTask/MY-CODE"
readonly CARGO_PKG_NAME="my-code"
readonly MIN_RUST_VERSION="1.75.0"
readonly RECOMMENDED_RUST_VERSION="1.80.0"

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly CYAN='\033[0;36m'
readonly MAGENTA='\033[0;35m'
readonly WHITE='\033[1;37m'
readonly NC='\033[0m' # No Color
readonly BOLD='\033[1m'
readonly DIM='\033[2m'

# Installation directories
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local}"
BIN_DIR="${INSTALL_DIR}/bin"
CONFIG_DIR="${CONFIG_DIR:-$HOME/.config/my-code}"
DATA_DIR="${DATA_DIR:-$HOME/.local/share/my-code}"
CACHE_DIR="${CACHE_DIR:-$HOME/.cache/my-code}"

# Feature flags
FEATURES="${FEATURES:-default}"
PROFILE="${PROFILE:-release}"

# =============================================================================
# Utility Functions
# =============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

log_debug() {
    if [[ "${DEBUG:-0}" == "1" ]]; then
        echo -e "${DIM}[DEBUG]${NC} $*"
    fi
}

log_step() {
    echo -e "\n${CYAN}━━━${NC} ${BOLD}$*${NC} ${CYAN}━━━${NC}"
}

print_banner() {
    echo ""
    echo "     __  __                ____                      "
    echo "    |  \/  | _____   _____|  _ \ ___  ___ ___  _ __  "
    echo "    | |\/| |/ _ \ \ / / _ \ |_) / _ \/ __/ _ \| '_ \ "
    echo "    | |  | | (_) \ V /  __/  __/ (_) | (_| (_) | | | |"
    echo "    |_|  |_|\___/ \_/ \___|_|   \___/ \___\___/|_| |_|"
    echo ""
    echo "    AI-powered terminal coding agent"
    echo -e "    Version: ${SCRIPT_VERSION}"
    echo ""
}

cleanup() {
    local exit_code=$?
    if [[ $exit_code -ne 0 ]]; then
        log_error "Installation failed with exit code: $exit_code"
        log_info "You can re-run the script with --debug flag for detailed logs"
    fi
    
    # Clean up temporary files
    if [[ -n "${TEMP_DIR:-}" && -d "${TEMP_DIR}" ]]; then
        log_debug "Cleaning up temporary directory: ${TEMP_DIR}"
        rm -rf "${TEMP_DIR}"
    fi
    
    exit $exit_code
}

trap cleanup EXIT INT TERM

# =============================================================================
# System Detection & Compatibility
# =============================================================================

detect_os() {
    local os
    case "$(uname -s)" in
        Linux*)     os="linux";;
        Darwin*)    os="macos";;
        CYGWIN*)    os="windows";;
        MINGW*)     os="windows";;
        MSYS*)      os="windows";;
        *)          os="unknown";;
    esac
    echo "$os"
}

detect_arch() {
    local arch
    case "$(uname -m)" in
        x86_64)     arch="x86_64";;
        amd64)      arch="x86_64";;
        arm64)      arch="aarch64";;
        aarch64)    arch="aarch64";;
        armv7l)     arch="armv7";;
        i386)       arch="i686";;
        i686)       arch="i686";;
        *)          arch="unknown";;
    esac
    echo "$arch"
}

get_shell_profile() {
    local shell_name
    shell_name=$(basename "${SHELL:-/bin/bash}")
    
    case "$shell_name" in
        bash)
            if [[ -f "$HOME/.bashrc" ]]; then
                echo "$HOME/.bashrc"
            else
                echo "$HOME/.bash_profile"
            fi
            ;;
        zsh)
            echo "$HOME/.zshrc"
            ;;
        fish)
            echo "$HOME/.config/fish/config.fish"
            ;;
        *)
            echo "$HOME/.profile"
            ;;
    esac
}

# =============================================================================
# Dependency Checks
# =============================================================================

check_command() {
    command -v "$1" &>/dev/null
}

check_rust() {
    if ! check_command cargo; then
        return 1
    fi
    
    local rust_version
    rust_version=$(cargo --version | awk '{print $2}')
    
    # Compare versions
    if [[ "$(printf '%s\n' "$MIN_RUST_VERSION" "$rust_version" | sort -V | head -n1)" != "$MIN_RUST_VERSION" ]]; then
        log_warning "Rust version $rust_version is below minimum required ($MIN_RUST_VERSION)"
        return 1
    fi
    
    return 0
}

check_dependencies() {
    log_step "Checking System Dependencies"
    
    local missing_deps=()
    local optional_deps=()
    
    # Required dependencies
    local required=("curl" "git")
    
    for dep in "${required[@]}"; do
        if ! check_command "$dep"; then
            missing_deps+=("$dep")
        else
            log_debug "✓ Found: $dep"
        fi
    done
    
    # Optional dependencies
    local optional=("pkg-config" "cmake" "libssl-dev" "build-essential")
    
    for dep in "${optional[@]}"; do
        if ! check_command "$dep"; then
            optional_deps+=("$dep")
        else
            log_debug "✓ Found (optional): $dep"
        fi
    done
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        
        if [[ "$(detect_os)" == "linux" ]]; then
            if check_command apt-get; then
                log_info "Install with: sudo apt-get install ${missing_deps[*]}"
            elif check_command dnf; then
                log_info "Install with: sudo dnf install ${missing_deps[*]}"
            elif check_command pacman; then
                log_info "Install with: sudo pacman -S ${missing_deps[*]}"
            fi
        elif [[ "$(detect_os)" == "macos" ]]; then
            log_info "Install with: brew install ${missing_deps[*]}"
        fi
        
        return 1
    fi
    
    if [[ ${#optional_deps[@]} -gt 0 ]]; then
        log_warning "Optional dependencies not found: ${optional_deps[*]}"
        log_info "These may be needed for building certain features"
    fi
    
    log_success "All required dependencies are installed"
    return 0
}

check_rust_installation() {
    log_step "Checking Rust Installation"
    
    if ! check_rust; then
        log_warning "Rust is not installed or version is too old"
        echo ""
        read -rp "Would you like to install Rust via rustup? [Y/n] " choice
        choice="${choice:-Y}"
        
        if [[ "$choice" =~ ^[Yy]$ ]]; then
            log_info "Installing Rust via rustup..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            
            # Source the cargo environment
            if [[ -f "$HOME/.cargo/env" ]]; then
                # shellcheck source=/dev/null
                source "$HOME/.cargo/env"
            fi
            
            if check_rust; then
                log_success "Rust installed successfully"
            else
                log_error "Failed to install Rust. Please install manually."
                return 1
            fi
        else
            log_error "Rust is required to build MY CODE from source"
            return 1
        fi
    else
        local rust_version
        rust_version=$(cargo --version)
        log_success "Found: $rust_version"
        
        # Check if update is recommended
        if [[ "$(printf '%s\n' "$RECOMMENDED_RUST_VERSION" "$rust_version" | sort -V | head -n1)" == "$RECOMMENDED_RUST_VERSION" ]]; then
            log_warning "Consider updating Rust to $RECOMMENDED_RUST_VERSION or newer for best performance"
        fi
    fi
    
    return 0
}

# =============================================================================
# Directory Setup
# =============================================================================

setup_directories() {
    log_step "Setting Up Directories"
    
    local dirs=("$INSTALL_DIR" "$BIN_DIR" "$CONFIG_DIR" "$DATA_DIR" "$CACHE_DIR")
    
    for dir in "${dirs[@]}"; do
        if [[ ! -d "$dir" ]]; then
            mkdir -p "$dir"
            log_debug "Created directory: $dir"
        else
            log_debug "Directory exists: $dir"
        fi
    done
    
    # Set appropriate permissions
    chmod 755 "$BIN_DIR"
    chmod 700 "$CONFIG_DIR"
    
    log_success "Directories configured"
}

# =============================================================================
# Build & Installation
# =============================================================================

build_from_source() {
    log_step "Building MY CODE from Source"
    
    local build_type="Debug"
    if [[ "$PROFILE" == "release" ]]; then
        build_type="Release"
    fi
    
    log_info "Building profile: $PROFILE"
    log_info "Features: $FEATURES"
    log_info "Build type: $build_type"
    
    # Create temporary directory for build artifacts
    TEMP_DIR=$(mktemp -d)
    
    # Clone repository if not in current directory
    if [[ ! -f "Cargo.toml" ]]; then
        log_info "Cloning repository..."
        git clone --depth 1 "https://github.com/${GITHUB_REPO}.git" "${TEMP_DIR}/source"
        cd "${TEMP_DIR}/source"
    fi
    
    # Build the project
    local cargo_args=("--profile" "$PROFILE")
    
    if [[ "$FEATURES" != "default" ]]; then
        cargo_args+=("--features" "$FEATURES")
    fi
    
    log_info "Running: cargo build ${cargo_args[*]}"
    cargo build "${cargo_args[@]}"
    
    # Install the binary
    log_info "Installing binary to $BIN_DIR"
    cp "target/${PROFILE}/${BINARY_NAME}" "${BIN_DIR}/"
    chmod +x "${BIN_DIR}/${BINARY_NAME}"
    
    log_success "Build completed successfully"
}

install_via_cargo() {
    log_step "Installing via Cargo"
    
    local cargo_args=("install" "$CARGO_PKG_NAME")
    
    if [[ "$PROFILE" == "release" ]]; then
        cargo_args+=("--locked")
    fi
    
    if [[ "$FEATURES" != "default" ]]; then
        cargo_args+=("--features" "$FEATURES")
    fi
    
    if [[ -n "${CARGO_INSTALL_ROOT:-}" ]]; then
        cargo_args+=("--root" "$CARGO_INSTALL_ROOT")
    else
        cargo_args+=("--root" "$INSTALL_DIR")
    fi
    
    log_info "Running: cargo ${cargo_args[*]}"
    cargo "${cargo_args[@]}"
    
    log_success "Installation completed via cargo"
}

# =============================================================================
# Configuration Setup
# =============================================================================

setup_configuration() {
    log_step "Setting Up Configuration"
    
    local config_file="${CONFIG_DIR}/config.toml"
    
    if [[ ! -f "$config_file" ]]; then
        cat > "$config_file" << 'CONFIGEOF'
# MY CODE Configuration File
# Generated by install.sh

[general]
# Default AI provider (openai, anthropic, google)
provider = "openai"

# Log level (error, warn, info, debug, trace)
log_level = "info"

# Enable telemetry (anonymous usage statistics)
telemetry = true

[api]
# API key for your chosen provider
# Recommended: Use environment variables instead
# OPENAI_API_KEY=your_key_here
# ANTHROPIC_API_KEY=your_key_here
# GOOGLE_API_KEY=your_key_here

[agent]
# Maximum context window size (tokens)
max_context = 8192

# Enable auto-save of sessions
auto_save = true

# Session history limit
history_limit = 100

[tools]
# Enable/disable specific tool categories
filesystem = true
terminal = true
git = true
search = true
code_analysis = true

[security]
# Require confirmation for file modifications
confirm_writes = true

# Require confirmation for command execution
confirm_commands = true

# Allowed directories for file operations
# allowed_dirs = ["~/projects", "~/work"]

[ui]
# Color scheme (dark, light, auto)
theme = "dark"

# Enable animations
animations = true

# Status bar position (top, bottom)
status_bar = "bottom"
CONFIGEOF
        log_success "Created default configuration at: $config_file"
    else
        log_info "Configuration already exists at: $config_file"
    fi
    
    # Create initial database
    local db_file="${DATA_DIR}/sessions.db"
    if [[ ! -f "$db_file" ]]; then
        log_info "Initializing session database..."
        touch "$db_file"
        log_success "Created session database at: $db_file"
    fi
}

setup_shell_integration() {
    log_step "Setting Up Shell Integration"
    
    local profile_file
    profile_file=$(get_shell_profile)
    
    # Check if BIN_DIR is already in PATH
    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        log_info "Adding $BIN_DIR to PATH in $profile_file"
        
        local export_line="export PATH=\"$BIN_DIR:\$PATH\""
        local comment="# Added by MY CODE installer on $(date '+%Y-%m-%d %H:%M:%S')"
        
        if [[ ! -f "$profile_file" ]] || ! grep -q "$BIN_DIR" "$profile_file" 2>/dev/null; then
            echo "" >> "$profile_file"
            echo "$comment" >> "$profile_file"
            echo "$export_line" >> "$profile_file"
            log_success "Added BIN_DIR to PATH"
        else
            log_info "PATH already configured in $profile_file"
        fi
        
        log_warning "Please restart your shell or run: source $profile_file"
    else
        log_info "BIN_DIR already in PATH"
    fi
    
    # Install completions
    local completion_dir
    case "$(basename "${SHELL:-/bin/bash}")" in
        bash)
            completion_dir="$HOME/.local/share/bash-completion/completions"
            ;;
        zsh)
            completion_dir="$HOME/.local/share/zsh/site-functions"
            ;;
        fish)
            completion_dir="$HOME/.config/fish/completions"
            ;;
        *)
            log_debug "Unknown shell, skipping completions"
            return 0
            ;;
    esac
    
    mkdir -p "$completion_dir"
    
    log_info "Generating shell completions..."
    if [[ -x "${BIN_DIR}/${BINARY_NAME}" ]]; then
        case "$(basename "${SHELL:-/bin/bash}")" in
            bash)
                "${BIN_DIR}/${BINARY_NAME}" --completions bash > "${completion_dir}/${BINARY_NAME}" 2>/dev/null || true
                ;;
            zsh)
                "${BIN_DIR}/${BINARY_NAME}" --completions zsh > "${completion_dir}/_${BINARY_NAME}" 2>/dev/null || true
                ;;
            fish)
                "${BIN_DIR}/${BINARY_NAME}" --completions fish > "${completion_dir}/${BINARY_NAME}.fish" 2>/dev/null || true
                ;;
        esac
        log_success "Shell completions installed"
    fi
}

# =============================================================================
# Verification & Testing
# =============================================================================

verify_installation() {
    log_step "Verifying Installation"
    
    local binary_path="${BIN_DIR}/${BINARY_NAME}"
    
    if [[ ! -x "$binary_path" ]]; then
        log_error "Binary not found or not executable: $binary_path"
        return 1
    fi
    
    log_info "Testing binary execution..."
    local version_output
    if version_output=$("$binary_path" --version 2>&1); then
        log_success "Binary verified: $version_output"
    else
        log_warning "Binary executed but --version failed (may need dependencies)"
    fi
    
    # Test help command
    if "$binary_path" --help &>/dev/null; then
        log_success "Help command works"
    else
        log_warning "Help command failed"
    fi
    
    return 0
}

run_post_install_tests() {
    log_step "Running Post-Installation Tests"
    
    local binary_path="${BIN_DIR}/${BINARY_NAME}"
    
    if [[ ! -x "$binary_path" ]]; then
        log_warning "Skipping tests - binary not found"
        return 0
    fi
    
    # Initialize test project
    log_info "Testing initialization..."
    local test_dir
    test_dir=$(mktemp -d)
    
    (
        cd "$test_dir"
        if "$binary_path" init &>/dev/null; then
            log_success "Initialization test passed"
        else
            log_warning "Initialization test skipped (requires API configuration)"
        fi
    )
    
    rm -rf "$test_dir"
    
    log_success "Post-installation tests completed"
}

# =============================================================================
# Uninstallation
# =============================================================================

uninstall() {
    log_step "Uninstalling MY CODE"
    
    read -rp "This will remove MY CODE from your system. Continue? [y/N] " choice
    choice="${choice:-N}"
    
    if [[ ! "$choice" =~ ^[Yy]$ ]]; then
        log_info "Uninstallation cancelled"
        return 0
    fi
    
    # Remove binary
    if [[ -f "${BIN_DIR}/${BINARY_NAME}" ]]; then
        rm "${BIN_DIR}/${BINARY_NAME}"
        log_success "Removed binary"
    fi
    
    # Remove configuration (optional)
    read -rp "Remove configuration files? [y/N] " remove_config
    if [[ "$remove_config" =~ ^[Yy]$ ]]; then
        if [[ -d "$CONFIG_DIR" ]]; then
            rm -rf "$CONFIG_DIR"
            log_success "Removed configuration directory"
        fi
        
        if [[ -d "$DATA_DIR" ]]; then
            rm -rf "$DATA_DIR"
            log_success "Removed data directory"
        fi
        
        if [[ -d "$CACHE_DIR" ]]; then
            rm -rf "$CACHE_DIR"
            log_success "Removed cache directory"
        fi
    fi
    
    # Remove from PATH (manual step)
    local profile_file
    profile_file=$(get_shell_profile)
    
    if [[ -f "$profile_file" ]]; then
        log_info "You may want to remove the following lines from $profile_file:"
        echo "  # Added by MY CODE installer"
        echo "  export PATH=\"$BIN_DIR:\$PATH\""
    fi
    
    log_success "Uninstallation completed"
}

# =============================================================================
# Help & Usage
# =============================================================================

show_help() {
    cat << EOF
${BOLD}MY CODE Installer${NC} - Version ${SCRIPT_VERSION}

${BOLD}USAGE:${NC}
    $0 [OPTIONS]

${BOLD}OPTIONS:${NC}
    -h, --help              Show this help message
    -v, --version           Show version information
    -u, --uninstall         Uninstall MY CODE
    -s, --skip-build        Skip building from source (use cargo install)
    -f, --features <FEAT>   Enable additional features (e.g., vendored-openssl)
    -p, --profile <TYPE>    Build profile: release (default) or debug
    -d, --debug             Enable debug output
    --no-config             Skip configuration setup
    --no-shell              Skip shell integration setup
    --prefix <DIR>          Installation prefix (default: ~/.local)
    --bin-dir <DIR>         Binary installation directory
    --config-dir <DIR>      Configuration directory
    -y, --yes               Accept all defaults without prompting

${BOLD}EXAMPLES:${NC}
    # Standard installation
    $0

    # Install with debug symbols
    $0 --profile debug

    # Install with vendored OpenSSL
    $0 --features vendored-openssl

    # Custom installation directory
    $0 --prefix /opt/my-code

    # Uninstall
    $0 --uninstall

${BOLD}ENVIRONMENT VARIABLES:${NC}
    INSTALL_DIR             Installation directory (default: ~/.local)
    CONFIG_DIR              Configuration directory (default: ~/.config/my-code)
    DATA_DIR                Data directory (default: ~/.local/share/my-code)
    CACHE_DIR               Cache directory (default: ~/.cache/my-code)
    FEATURES                Cargo features to enable
    PROFILE                 Build profile (release/debug)
    DEBUG                   Enable debug mode (set to 1)

${BOLD}POST-INSTALLATION:${NC}
    1. Restart your shell or run: source $(get_shell_profile)
    2. Configure your API keys in: ${CONFIG_DIR}/config.toml
    3. Run: ${BINARY_NAME} --help to see available commands
    4. Start with: ${BINARY_NAME} chat

${BOLD}SUPPORT:${NC}
    Documentation: https://github.com/${GITHUB_REPO}
    Issues: https://github.com/${GITHUB_REPO}/issues

EOF
}

# =============================================================================
# Main Installation Flow
# =============================================================================

main() {
    print_banner
    
    # Parse arguments
    local skip_build=false
    local do_uninstall=false
    local no_config=false
    local no_shell=false
    local accept_all=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -v|--version)
                echo "MY CODE Installer v${SCRIPT_VERSION}"
                exit 0
                ;;
            -u|--uninstall)
                do_uninstall=true
                shift
                ;;
            -s|--skip-build)
                skip_build=true
                shift
                ;;
            -f|--features)
                FEATURES="$2"
                shift 2
                ;;
            -p|--profile)
                PROFILE="$2"
                shift 2
                ;;
            -d|--debug)
                DEBUG=1
                set -x
                shift
                ;;
            --no-config)
                no_config=true
                shift
                ;;
            --no-shell)
                no_shell=true
                shift
                ;;
            --prefix)
                INSTALL_DIR="$2"
                BIN_DIR="${INSTALL_DIR}/bin"
                shift 2
                ;;
            --bin-dir)
                BIN_DIR="$2"
                shift 2
                ;;
            --config-dir)
                CONFIG_DIR="$2"
                shift 2
                ;;
            -y|--yes)
                accept_all=true
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # Handle uninstall
    if [[ "$do_uninstall" == true ]]; then
        uninstall
        exit 0
    fi
    
    # Show installation summary
    log_step "Installation Summary"
    echo "  Target Directory: ${BIN_DIR}"
    echo "  Config Directory: ${CONFIG_DIR}"
    echo "  Data Directory: ${DATA_DIR}"
    echo "  Build Profile: ${PROFILE}"
    echo "  Features: ${FEATURES}"
    echo ""
    
    if [[ "$accept_all" != true ]]; then
        read -rp "Continue with installation? [Y/n] " choice
        choice="${choice:-Y}"
        
        if [[ ! "$choice" =~ ^[Yy]$ ]]; then
            log_info "Installation cancelled"
            exit 0
        fi
    fi
    
    # Run installation steps
    check_dependencies
    check_rust_installation
    setup_directories
    
    # Build or install
    if [[ "$skip_build" == true ]]; then
        install_via_cargo
    else
        build_from_source
    fi
    
    # Configuration
    if [[ "$no_config" != true ]]; then
        setup_configuration
    fi
    
    # Shell integration
    if [[ "$no_shell" != true ]]; then
        setup_shell_integration
    fi
    
    # Verification
    verify_installation
    
    # Final summary
    log_step "Installation Complete! 🎉"
    
    cat << EOF
    
${GREEN}MY CODE has been successfully installed!${NC}

${BOLD}Next Steps:${NC}
  1. Add to PATH (if not automatic):
     export PATH="${BIN_DIR}:\$PATH"
  
  2. Configure your API keys:
     ${CONFIG_DIR}/config.toml
  
  3. Get started:
     ${BINARY_NAME} --help
     ${BINARY_NAME} chat
     ${BINARY_NAME} plan "add user authentication"

${BOLD}Documentation:${NC}
  https://github.com/${GITHUB_REPO}

${DIM}Tip: Run '${BINARY_NAME} init' in your project directory to get started${NC}

EOF
    
    log_success "Happy coding! 🚀"
}

# Run main function
main "$@"
