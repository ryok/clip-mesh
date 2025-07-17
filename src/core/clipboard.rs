use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info};

use crate::core::clip_item::ClipItem;
use crate::storage::Storage;
use crate::transforms::TransformEngine;

pub struct ClipboardMonitor {
    clipboard: Arc<Mutex<ClipboardContext>>,
    last_content: Arc<Mutex<Option<String>>>,
    device_id: String,
    transform_engine: TransformEngine,
}

impl ClipboardMonitor {
    pub fn new() -> Result<Self> {
        let clipboard = ClipboardContext::new()
            .map_err(|e| anyhow::anyhow!("Failed to create clipboard context: {:?}", e))?;
        
        let device_id = Self::generate_device_id();
        
        Ok(Self {
            clipboard: Arc::new(Mutex::new(clipboard)),
            last_content: Arc::new(Mutex::new(None)),
            device_id,
            transform_engine: TransformEngine::new(),
        })
    }
    
    fn generate_device_id() -> String {
        // In a real implementation, this would be persisted
        format!("{}-{}", 
            std::env::consts::OS,
            uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
        )
    }
    
    pub async fn start(self, storage: Storage) -> Result<()> {
        info!("Starting clipboard monitor on device: {}", self.device_id);
        
        loop {
            match self.check_clipboard(&storage).await {
                Ok(changed) => {
                    if changed {
                        debug!("Clipboard content changed");
                    }
                }
                Err(e) => {
                    error!("Error checking clipboard: {}", e);
                }
            }
            
            sleep(Duration::from_millis(500)).await;
        }
    }
    
    async fn check_clipboard(&self, storage: &Storage) -> Result<bool> {
        let mut clipboard = self.clipboard.lock().await;
        
        let current_content = match clipboard.get_contents() {
            Ok(content) => content,
            Err(e) => {
                // Clipboard is empty or inaccessible - this is normal
                debug!("Clipboard empty or inaccessible: {:?}", e);
                return Ok(false);
            }
        };
        
        let mut last_content = self.last_content.lock().await;
        
        if last_content.as_ref() != Some(&current_content) && !current_content.is_empty() {
            // Content has changed
            let mut clip_item = ClipItem::new(current_content.clone(), self.device_id.clone());
            
            info!("New clipboard item: {:?}", clip_item.content_type);
            
            // Apply smart transformations
            if let Err(e) = self.transform_engine.apply_smart_transforms(&mut clip_item).await {
                error!("Failed to apply transformations: {}", e);
            } else if !clip_item.transformations.is_empty() {
                info!("Applied {} transformations", clip_item.transformations.len());
            }
            
            // Store the item
            storage.store_item(clip_item).await?;
            
            // Update last content
            *last_content = Some(current_content);
            
            return Ok(true);
        }
        
        Ok(false)
    }
}