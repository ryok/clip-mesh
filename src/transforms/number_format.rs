use crate::core::clip_item::{ClipItem, ContentType, TransformType};
use crate::transforms::Transform;
use anyhow::Result;

pub struct NumberFormatter;

impl NumberFormatter {
    pub fn new() -> Self {
        Self
    }
    
    fn format_with_commas(n: i64) -> String {
        let s = n.abs().to_string();
        let chunks: Vec<&str> = s
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();
        
        let formatted = chunks.join(",");
        if n < 0 {
            format!("-{}", formatted)
        } else {
            formatted
        }
    }
}

impl Transform for NumberFormatter {
    fn can_transform(&self, item: &ClipItem) -> bool {
        matches!(item.content_type, ContentType::Number)
    }
    
    fn transform(&self, content: &str) -> Result<String> {
        let number: f64 = content.trim().parse()?;
        
        // Format with thousands separator
        let formatted = if number.fract() == 0.0 {
            // Format as integer
            let int_part = number as i64;
            Self::format_with_commas(int_part)
        } else {
            // Format with 2 decimal places
            let int_part = number.trunc() as i64;
            let fract_str = format!("{:.2}", number.fract());
            let decimal_part = fract_str.trim_start_matches("0.");
            format!("{}.{}", Self::format_with_commas(int_part), decimal_part)
        };
        
        Ok(formatted)
    }
    
    fn transform_type(&self) -> TransformType {
        TransformType::NumberFormat
    }
}