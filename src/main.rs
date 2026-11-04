//! SigmaOS-compositor - SigmaOS Component CLI
//!
//! Command-line interface for SigmaOS-compositor component.

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use compositor::{Component, Config, Compositor};

#[derive(Parser)]
#[command(name = "compositor")]
#[command(about = "SigmaOS SigmaOS-compositor component", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the component
    Init {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Start the component
    Start,
    /// Stop the component
    Stop,
    /// Get component status
    Status,
    /// Run the component
    Run {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::default();
    let mut component = Compositor::new(config);

    match cli.command {
        Commands::Init { debug } => {
            if debug {
                println!("Initializing SigmaOS-compositor in debug mode...");
            } else {
                println!("Initializing SigmaOS-compositor...");
            }
            component.initialize()?;
            println!("SigmaOS-compositor initialized successfully");
        }
        Commands::Start => {
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-compositor started successfully");
        }
        Commands::Stop => {
            component.stop()?;
            println!("SigmaOS-compositor stopped successfully");
        }
        Commands::Status => {
            let status = component.status();
            println!("SigmaOS-compositor status: {:?}", status);
        }
        Commands::Run { debug } => {
            if debug {
                println!("Running SigmaOS-compositor in debug mode...");
            } else {
                println!("Running SigmaOS-compositor...");
            }
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-compositor is running. Press Ctrl+C to stop.");
            
            // Keep running
            tokio::runtime::Runtime::new()?.block_on(async {
                let _ = tokio::signal::ctrl_c().await;
                info!("Received Ctrl+C, shutting down...");
            });
            
            component.stop()?;
            println!("SigmaOS-compositor stopped");
        }
    }

    Ok(())
}
