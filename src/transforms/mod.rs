pub mod number_format;
pub mod text_transform;

use crate::core::clip_item::{ClipItem, Transformation, TransformType};
use anyhow::Result;

pub trait Transform {
    fn can_transform(&self, item: &ClipItem) -> bool;
    fn transform(&self, content: &str) -> Result<String>;
    fn transform_type(&self) -> TransformType;
}

pub struct TransformEngine {
    transforms: Vec<Box<dyn Transform + Send + Sync>>,
}

impl TransformEngine {
    pub fn new() -> Self {
        let transforms: Vec<Box<dyn Transform + Send + Sync>> = vec![
            Box::new(number_format::NumberFormatter::new()),
            Box::new(text_transform::TextTransformer::new()),
        ];
        
        Self { transforms }
    }
    
    pub async fn apply_smart_transforms(&self, item: &mut ClipItem) -> Result<()> {
        for transform in &self.transforms {
            if transform.can_transform(item) {
                match transform.transform(&item.content) {
                    Ok(result) => {
                        let transformation = Transformation {
                            transform_type: transform.transform_type(),
                            result,
                            timestamp: chrono::Utc::now(),
                        };
                        item.transformations.push(transformation);
                    }
                    Err(e) => {
                        tracing::warn!("Transform failed: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    }
}