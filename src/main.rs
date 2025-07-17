mod cli;
mod core;
mod storage;
mod transforms;
mod ui;

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};
use tracing_subscriber;

use cli::{Cli, Commands, CliHandler};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .init();

    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Monitor) | None => {
            // Start monitoring mode
            info!("Starting ClipMesh monitoring...");

            let monitor = core::clipboard::ClipboardMonitor::new()?;
            let storage = storage::Storage::new().await?;
            
            tokio::select! {
                result = monitor.start(storage) => {
                    if let Err(e) = result {
                        error!("Clipboard monitor error: {}", e);
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    info!("Shutting down ClipMesh...");
                }
            }
        }
        Some(command) => {
            // Handle CLI commands
            let handler = CliHandler::new().await?;
            handler.handle_command(command).await?;
        }
    }

    Ok(())
}
