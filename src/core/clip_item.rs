use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipItem {
    pub id: String,
    pub content: String,
    pub content_type: ContentType,
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub tags: Vec<String>,
    pub transformations: Vec<Transformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Text,
    Url,
    Code,
    Number,
    Date,
    Address,
    Email,
    Json,
    Csv,
    Image,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    pub transform_type: TransformType,
    pub result: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformType {
    Translation { from: String, to: String },
    NumberFormat,
    DateFormat,
    JsonPretty,
    CsvToMarkdown,
    Summary,
    ToneAdjustment { tone: String },
}

impl ClipItem {
    pub fn new(content: String, device_id: String) -> Self {
        let content_type = Self::detect_content_type(&content);
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            content_type,
            timestamp: Utc::now(),
            device_id,
            tags: Vec::new(),
            transformations: Vec::new(),
        }
    }
    
    fn detect_content_type(content: &str) -> ContentType {
        let trimmed = content.trim();
        
        // URL detection
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return ContentType::Url;
        }
        
        // Email detection
        if trimmed.contains('@') && trimmed.contains('.') && !trimmed.contains(' ') {
            return ContentType::Email;
        }
        
        // Number detection
        if trimmed.parse::<f64>().is_ok() {
            return ContentType::Number;
        }
        
        // JSON detection
        if (trimmed.starts_with('{') && trimmed.ends_with('}')) ||
           (trimmed.starts_with('[') && trimmed.ends_with(']')) {
            return ContentType::Json;
        }
        
        // CSV detection (simple check)
        if trimmed.lines().count() > 1 && trimmed.contains(',') {
            return ContentType::Csv;
        }
        
        ContentType::Text
    }
}