use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::models::{ApiInfo, EndpointInfo, HealthResponse};

pub async fn health_check() -> HttpResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn api_info() -> HttpResponse {
    let info = ApiInfo {
        name: "Glastocam API".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Backend API for the Glastonbury Festival webcam viewer".to_string(),
        endpoints: vec![
            EndpointInfo {
                path: "/api/v1/health".to_string(),
                method: "GET".to_string(),
                description: "Health check endpoint".to_string(),
            },
            EndpointInfo {
                path: "/api/v1/info".to_string(),
                method: "GET".to_string(),
                description: "API information and available endpoints".to_string(),
            },
            EndpointInfo {
                path: "/api/v1/webcam".to_string(),
                method: "GET".to_string(),
                description: "Get webcam configuration and metadata".to_string(),
            },
            EndpointInfo {
                path: "/api/v1/webcam/url".to_string(),
                method: "GET".to_string(),
                description: "Get the current webcam image URL with cache-busting".to_string(),
            },
            EndpointInfo {
                path: "/api/v1/webcam/proxy".to_string(),
                method: "GET".to_string(),
                description: "Proxy the webcam image through the backend".to_string(),
            },
        ],
    };

    HttpResponse::Ok().json(info)
}

