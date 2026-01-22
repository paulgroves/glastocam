use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebcamInfo {
    pub url: String,
    pub refresh_interval_secs: u64,
    pub last_updated: DateTime<Utc>,
    pub cache_bust_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub path: String,
    pub method: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            timestamp: Utc::now(),
        }
    }
}

