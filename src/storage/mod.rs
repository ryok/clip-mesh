use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::core::clip_item::ClipItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageData {
    pub items: Vec<ClipItem>,
    pub last_updated: DateTime<Utc>,
}

pub struct Storage {
    data_path: PathBuf,
    data: Arc<RwLock<StorageData>>,
}

impl Storage {
    pub async fn new() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?
            .join("clip-mesh");
        
        // Create directory if it doesn't exist
        fs::create_dir_all(&data_dir).await?;
        
        let data_path = data_dir.join("clipboard_history.json");
        
        // Load existing data or create new
        let data = if data_path.exists() {
            let content = fs::read_to_string(&data_path).await?;
            serde_json::from_str(&content)?
        } else {
            StorageData {
                items: Vec::new(),
                last_updated: Utc::now(),
            }
        };
        
        Ok(Self {
            data_path,
            data: Arc::new(RwLock::new(data)),
        })
    }
    
    pub async fn store_item(&self, item: ClipItem) -> Result<()> {
        let mut data = self.data.write().await;
        
        // Add to beginning (most recent first)
        data.items.insert(0, item);
        
        // Keep only last 1000 items
        if data.items.len() > 1000 {
            data.items.truncate(1000);
        }
        
        data.last_updated = Utc::now();
        
        // Save to disk
        self.save_data(&data).await?;
        
        Ok(())
    }
    
    pub async fn get_items(&self, limit: Option<usize>) -> Result<Vec<ClipItem>> {
        let data = self.data.read().await;
        let items = if let Some(limit) = limit {
            data.items.iter().take(limit).cloned().collect()
        } else {
            data.items.clone()
        };
        Ok(items)
    }
    
    pub async fn search_items(&self, query: &str) -> Result<Vec<ClipItem>> {
        let data = self.data.read().await;
        let query_lower = query.to_lowercase();
        
        let results: Vec<ClipItem> = data.items
            .iter()
            .filter(|item| item.content.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();
        
        Ok(results)
    }
    
    async fn save_data(&self, data: &StorageData) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.data_path, json).await?;
        Ok(())
    }
}