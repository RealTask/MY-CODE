//! MY CODE - AI-powered terminal coding agent
//!
//! A professional developer tool that allows developers to interact with their codebase
//! using natural language.

use anyhow::Result;
use clap::Parser;
use my_code::cli::{CliArgs, Command};
use my_code::config::Config;
use my_code::errors::app::AppError;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Initialize logging
    init_logging(&args);

    // Load configuration
    let config = Config::load(args.config.as_deref())?;

    // Execute command
    match &args.command {
        Some(cmd) => execute_command(cmd, &args, &config)?,
        None => {
            // Default to chat if no command specified
            execute_command(&Command::Chat { message: vec![] }, &args, &config)?;
        }
    }

    Ok(())
}

/// Initialize logging based on CLI arguments
fn init_logging(args: &CliArgs) {
    let log_level = args.log_level();

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false);

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer);

    subscriber.init();
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
                println!("Task: {}", msg);
                println!("\nStarting interactive session...");
                // TODO: Implement full chat session
            } else {
                println!("Starting interactive chat session...");
                println!("Type your request or 'quit' to exit.\n");
                // TODO: Implement interactive loop
            }
            
            Ok(())
        }
        
        Command::Plan { description } => {
            println!("📋 Planning: {}", description);
            // TODO: Implement planning
            Ok(())
        }
        
        Command::Build { description } => {
            println!("🔨 Building: {}", description);
            // TODO: Implement build
            Ok(())
        }
        
        Command::Review { path } => {
            println!("🔍 Reviewing changes...");
            if let Some(p) = path {
                println!("Path: {}", p);
            }
            // TODO: Implement review
            Ok(())
        }
        
        Command::Debug { description } => {
            println!("🐛 Debugging: {}", description);
            // TODO: Implement debug
            Ok(())
        }
        
        Command::Test { pattern } => {
            println!("🧪 Running tests...");
            if let Some(p) = pattern {
                println!("Pattern: {}", p);
            }
            // TODO: Implement test
            Ok(())
        }
        
        Command::Explain { target } => {
            println!("📖 Explaining: {}", target);
            // TODO: Implement explain
            Ok(())
        }
        
        Command::Diff => {
            println!("📊 Showing diff...");
            // TODO: Implement diff
            Ok(())
        }
        
        Command::Undo { count } => {
            println!("↩️  Undoing {} change(s)...", count);
            // TODO: Implement undo
            Ok(())
        }
        
        Command::Init => {
            println!("🚀 Initializing MY CODE in project...");
            // TODO: Implement init
            Ok(())
        }
        
        Command::Config { key, value } => {
            println!("⚙️  Configuration");
            match (key, value) {
                (Some(k), Some(v)) => println!("Setting {} = {}", k, v),
                (Some(k), None) => println!("Getting {}", k),
                (None, _) => println!("Opening configuration editor..."),
            }
            // TODO: Implement config
            Ok(())
        }
        
        Command::Doctor => {
            println!("🏥 Running diagnostics...\n");
            run_doctor()?;
            Ok(())
        }
        
        Command::Sessions => {
            println!("📁 Sessions");
            // TODO: List sessions
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
fn run_doctor() -> Result<()> {
    use std::process::Command as StdCommand;
    
    println!("Checking system setup...\n");
    
    // Check Rust installation
    print!("Rust: ");
    if StdCommand::new("rustc").arg("--version").output().is_ok() {
        println!("✓ installed");
    } else {
        println!("✗ not found");
    }
    
    // Check Cargo
    print!("Cargo: ");
    if StdCommand::new("cargo").arg("--version").output().is_ok() {
        println!("✓ installed");
    } else {
        println!("✗ not found");
    }
    
    // Check Git
    print!("Git: ");
    if StdCommand::new("git").arg("--version").output().is_ok() {
        println!("✓ installed");
    } else {
        println!("✗ not found");
    }
    
    // Check ripgrep
    print!("ripgrep: ");
    if StdCommand::new("rg").arg("--version").output().is_ok() {
        println!("✓ installed");
    } else {
        println!("✗ not found (optional)");
    }
    
    // Check current directory
    print!("Current directory: ");
    if let Ok(pwd) = std::env::current_dir() {
        println!("✓ {}", pwd.display());
    } else {
        println!("✗ cannot determine");
    }
    
    // Check write permissions
    print!("Write permissions: ");
    let test_file = std::env::current_dir().ok().map(|p| p.join(".my-code-test"));
    if let Some(file) = &test_file {
        if std::fs::write(file, "").and_then(|_| std::fs::remove_file(file)).is_ok() {
            println!("✓ ok");
        } else {
            println!("✗ denied");
        }
    } else {
        println!("? unknown");
    }
    
    println!("\n✅ Diagnostics complete");
    
    Ok(())
}
