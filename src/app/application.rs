//! Application lifecycle coordinator

use anyhow::Result;

use crate::cli::Command;
use crate::config::Config;
use crate::database::Database;
use crate::events::EventBus;
use crate::sessions::SessionManager;

/// Central application coordinator
pub struct Application {
    config: Config,
    session_manager: SessionManager,
    database: Database,
    event_bus: EventBus,
}

impl Application {
    /// Create a new application instance
    pub fn new(config: Config) -> Result<Self> {
        let database = Database::new()?;
        let session_manager = SessionManager::new(&database)?;
        let event_bus = EventBus::new();

        Ok(Self {
            config,
            session_manager,
            database,
            event_bus,
        })
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the session manager
    pub fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }

    /// Get the database
    pub fn database(&self) -> &Database {
        &self.database
    }

    /// Get the event bus
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Run the application with the given command
    pub async fn run(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Chat { message } => {
                if message.is_empty() {
                    println!("Starting interactive chat session...");
                    println!("Type your request or 'quit' to exit.\n");
                } else {
                    println!("Task: {}", message.join(" "));
                    println!("\nStarting interactive session...");
                }
            }
            Command::Plan { description } => {
                println!("📋 Planning: {description}");
            }
            Command::Build { description } => {
                println!("🔨 Building: {description}");
            }
            Command::Review { path } => {
                println!("🔍 Reviewing changes...");
                if let Some(p) = path {
                    println!("Path: {p}");
                }
            }
            Command::Debug { description } => {
                println!("🐛 Debugging: {description}");
            }
            Command::Test { pattern } => {
                println!("🧪 Running tests...");
                if let Some(p) = pattern {
                    println!("Pattern: {p}");
                }
            }
            Command::Explain { target } => {
                println!("📖 Explaining: {target}");
            }
            Command::Diff => {
                println!("📊 Showing diff...");
            }
            Command::Undo { count } => {
                println!("↩️  Undoing {count} change(s)...");
            }
            Command::Init => {
                println!("🚀 Initializing MY CODE in project...");
                init_project()?;
            }
            Command::Config { key, value } => match (key, value) {
                (Some(k), Some(v)) => println!("Setting {k} = {v}"),
                (Some(k), None) => println!("Getting {k}"),
                (None, _) => println!("Opening configuration editor..."),
            },
            Command::Doctor => {
                println!("🏥 Running diagnostics...\n");
            }
            Command::Sessions => {
                let sessions = self.session_manager.list();
                if sessions.is_empty() {
                    println!("No sessions found.");
                } else {
                    println!("📁 Sessions");
                    for session in sessions {
                        println!("  {}  {}", session.id, session.title);
                    }
                }
            }
            Command::Version => {
                println!("MY CODE v{}", env!("CARGO_PKG_VERSION"));
            }
            Command::Completions { shell } => {
                use clap::CommandFactory;
                use clap_complete::generate;
                use std::io;

                let mut cmd = crate::cli::CliArgs::command();
                generate(shell, &mut cmd, "my-code", &mut io::stdout());
            }
        }
        Ok(())
    }

    /// Shutdown the application gracefully
    pub async fn shutdown(&mut self) -> Result<()> {
        self.database.close()?;
        Ok(())
    }
}

fn init_project() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let config_path = cwd.join("my-code.toml");
    if !config_path.exists() {
        crate::config::Config::default().save(&config_path)?;
        println!("Created {}", config_path.display());
    } else {
        println!("Configuration already exists at {}", config_path.display());
    }
    Ok(())
}
