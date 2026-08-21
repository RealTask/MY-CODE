//! MY CODE - AI-powered terminal coding agent
//!
//! A professional developer tool that allows developers to interact with their codebase
//! using natural language.

use anyhow::Result;
use clap::{CommandFactory, Parser};
use my_code::cli::{CliArgs, Command};
use my_code::config::Config;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    init_logging(&args);

    let mut config = Config::load(args.config.as_deref())?;
    if let Some(model) = &args.model {
        config.default_model = Some(model.clone());
    }
    if let Some(provider) = &args.provider {
        config.default_provider = provider.clone();
    }

    match &args.command {
        Some(cmd) => execute_command(cmd, &args, &config)?,
        None => {
            execute_command(&Command::Chat { message: vec![] }, &args, &config)?;
        }
    }

    Ok(())
}

/// Initialize logging based on CLI arguments
fn init_logging(args: &CliArgs) {
    let log_level = args.log_level();

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}

/// Execute the specified command
fn execute_command(cmd: &Command, args: &CliArgs, config: &Config) -> Result<()> {
    match cmd {
        Command::Chat { message } => {
            let initial_message = if message.is_empty() {
                None
            } else {
                Some(message.join(" "))
            };

            println!("🤖 MY CODE - AI Coding Agent");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            if let Some(msg) = initial_message {
                println!("Task: {msg}");
                println!("\nStarting interactive session...");
            } else {
                println!("Starting interactive chat session...");
                println!("Type your request or 'quit' to exit.\n");
            }

            Ok(())
        }

        Command::Plan { description } => {
            println!("📋 Planning: {description}");
            Ok(())
        }

        Command::Build { description } => {
            println!("🔨 Building: {description}");
            Ok(())
        }

        Command::Review { path } => {
            println!("🔍 Reviewing changes...");
            if let Some(p) = path {
                println!("Path: {p}");
            }
            Ok(())
        }

        Command::Debug { description } => {
            println!("🐛 Debugging: {description}");
            Ok(())
        }

        Command::Test { pattern } => {
            println!("🧪 Running tests...");
            if let Some(p) = pattern {
                println!("Pattern: {p}");
            }
            Ok(())
        }

        Command::Explain { target } => {
            println!("📖 Explaining: {target}");
            Ok(())
        }

        Command::Diff => {
            println!("📊 Showing diff...");
            Ok(())
        }

        Command::Undo { count } => {
            println!("↩️  Undoing {count} change(s)...");
            Ok(())
        }

        Command::Init => {
            println!("🚀 Initializing MY CODE in project...");
            let cwd = std::env::current_dir()?;
            let config_path = cwd.join("my-code.toml");
            if config_path.exists() {
                println!("Configuration already exists at {}", config_path.display());
            } else {
                config.save(&config_path)?;
                println!("Created {}", config_path.display());
            }
            Ok(())
        }

        Command::Config { key, value } => {
            println!("⚙️  Configuration");
            match (key, value) {
                (Some(k), Some(v)) => println!("Setting {k} = {v}"),
                (Some(k), None) => println!("Getting {k}"),
                (None, _) => println!("Opening configuration editor..."),
            }
            Ok(())
        }

        Command::Doctor => {
            println!("🏥 Running diagnostics...\n");
            run_doctor(args, config)?;
            Ok(())
        }

        Command::Sessions => {
            println!("📁 Sessions");
            Ok(())
        }

        Command::Version => {
            println!("MY CODE v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }

        Command::Completions { shell } => {
            use clap_complete::generate;
            use std::io;

            let mut cmd = CliArgs::command();
            generate(*shell, &mut cmd, "my-code", &mut io::stdout());
            Ok(())
        }
    }
}

/// Run diagnostics and check setup
fn run_doctor(args: &CliArgs, config: &Config) -> Result<()> {
    use std::process::Command as StdCommand;

    println!("Checking system setup...\n");

    print!("Rust: ");
    match StdCommand::new("rustc").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("✓ {}", String::from_utf8_lossy(&output.stdout).trim());
        }
        _ => println!("✗ not found"),
    }

    print!("Cargo: ");
    match StdCommand::new("cargo").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("✓ {}", String::from_utf8_lossy(&output.stdout).trim());
        }
        _ => println!("✗ not found"),
    }

    print!("Git: ");
    match StdCommand::new("git").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("✓ {}", String::from_utf8_lossy(&output.stdout).trim());
        }
        _ => println!("✗ not found"),
    }

    print!("ripgrep: ");
    if StdCommand::new("rg").arg("--version").output().is_ok() {
        println!("✓ installed");
    } else {
        println!("✗ not found (optional)");
    }

    print!("Current directory: ");
    if let Ok(pwd) = std::env::current_dir() {
        println!("✓ {}", pwd.display());
    } else {
        println!("✗ cannot determine");
    }

    print!("Write permissions: ");
    let test_file = std::env::current_dir().ok().map(|p| p.join(".my-code-test"));
    if let Some(file) = &test_file {
        if std::fs::write(file, b"").and_then(|_| std::fs::remove_file(file)).is_ok() {
            println!("✓ ok");
        } else {
            println!("✗ denied");
        }
    } else {
        println!("? unknown");
    }

    print!("Config: ");
    println!(
        "✓ provider={} model={}",
        config.default_provider,
        config.default_model.as_deref().unwrap_or("(default)")
    );

    if args.debug {
        println!("Debug logging: enabled");
    }

    println!("\n✅ Diagnostics complete");

    Ok(())
}
