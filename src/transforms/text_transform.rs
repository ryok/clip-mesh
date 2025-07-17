use crate::core::clip_item::{ClipItem, ContentType, TransformType};
use crate::transforms::Transform;
use anyhow::Result;

pub struct TextTransformer;

impl TextTransformer {
    pub fn new() -> Self {
        Self
    }
}

impl Transform for TextTransformer {
    fn can_transform(&self, item: &ClipItem) -> bool {
        matches!(item.content_type, ContentType::Text | ContentType::Email | ContentType::Url)
    }
    
    fn transform(&self, content: &str) -> Result<String> {
        // For now, just implement basic transformations
        // In the future, this would use AI models
        
        // Simple example: trim and clean up whitespace
        let cleaned = content
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        
        Ok(cleaned)
    }
    
    fn transform_type(&self) -> TransformType {
        TransformType::ToneAdjustment { tone: "cleaned".to_string() }
    }
}