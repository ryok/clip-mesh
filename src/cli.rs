use clap::{Parser, Subcommand};
use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::storage::Storage;

#[derive(Parser)]
#[command(name = "clip-mesh")]
#[command(about = "AI-powered universal clipboard manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start clipboard monitoring daemon
    Monitor,
    /// List clipboard history
    List {
        /// Number of items to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Search clipboard history
    Search {
        /// Search query
        query: String,
    },
    /// Show detailed information about a specific item
    Show {
        /// Item ID
        id: String,
    },
}

pub struct CliHandler {
    storage: Storage,
}

impl CliHandler {
    pub async fn new() -> Result<Self> {
        let storage = Storage::new().await?;
        Ok(Self { storage })
    }
    
    pub async fn handle_command(&self, command: Commands) -> Result<()> {
        match command {
            Commands::Monitor => {
                // This will be handled by main.rs
                Ok(())
            }
            Commands::List { limit } => {
                self.list_items(limit).await
            }
            Commands::Search { query } => {
                self.search_items(&query).await
            }
            Commands::Show { id } => {
                self.show_item(&id).await
            }
        }
    }
    
    async fn list_items(&self, limit: usize) -> Result<()> {
        let items = self.storage.get_items(Some(limit)).await?;
        
        if items.is_empty() {
            println!("No clipboard history found.");
            return Ok(());
        }
        
        println!("Clipboard History (last {} items):", items.len());
        println!("{}", "─".repeat(80));
        
        for (index, item) in items.iter().enumerate() {
            let timestamp = item.timestamp.format("%Y-%m-%d %H:%M:%S");
            let preview = if item.content.len() > 50 {
                format!("{}...", &item.content[..50])
            } else {
                item.content.clone()
            };
            let preview = preview.replace('\n', " ");
            
            println!("{:2}. [{}] {:?} - {}", 
                index + 1, 
                timestamp, 
                item.content_type, 
                preview
            );
            
            if !item.transformations.is_empty() {
                println!("    Transformations: {}", item.transformations.len());
            }
        }
        
        Ok(())
    }
    
    async fn search_items(&self, query: &str) -> Result<()> {
        let items = self.storage.search_items(query).await?;
        
        if items.is_empty() {
            println!("No items found matching '{}'", query);
            return Ok(());
        }
        
        println!("Search results for '{}' ({} items):", query, items.len());
        println!("{}", "─".repeat(80));
        
        for (index, item) in items.iter().enumerate() {
            let timestamp = item.timestamp.format("%Y-%m-%d %H:%M:%S");
            let preview = if item.content.len() > 50 {
                format!("{}...", &item.content[..50])
            } else {
                item.content.clone()
            };
            let preview = preview.replace('\n', " ");
            
            println!("{:2}. [{}] {:?} - {}", 
                index + 1, 
                timestamp, 
                item.content_type, 
                preview
            );
        }
        
        Ok(())
    }
    
    async fn show_item(&self, id: &str) -> Result<()> {
        let items = self.storage.get_items(None).await?;
        
        if let Some(item) = items.iter().find(|item| item.id == id) {
            println!("Item Details:");
            println!("ID: {}", item.id);
            println!("Type: {:?}", item.content_type);
            println!("Timestamp: {}", item.timestamp.format("%Y-%m-%d %H:%M:%S"));
            println!("Device: {}", item.device_id);
            println!("Content:");
            println!("{}", "─".repeat(40));
            println!("{}", item.content);
            println!("{}", "─".repeat(40));
            
            if !item.transformations.is_empty() {
                println!("\nTransformations:");
                for (index, transform) in item.transformations.iter().enumerate() {
                    println!("{}. {:?} -> {}", 
                        index + 1, 
                        transform.transform_type, 
                        transform.result
                    );
                }
            }
            
            if !item.tags.is_empty() {
                println!("\nTags: {}", item.tags.join(", "));
            }
        } else {
            println!("Item with ID '{}' not found", id);
        }
        
        Ok(())
    }
}