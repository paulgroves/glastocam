use actix_web::{web, HttpResponse};
use chrono::Utc;
use log::{error, info};

use crate::config::Config;
use crate::models::{ErrorResponse, WebcamInfo};

pub async fn get_webcam_info(config: web::Data<Config>) -> HttpResponse {
    let now = Utc::now();
    let timestamp = now.timestamp_millis();

    let info = WebcamInfo {
        url: config.webcam_url.clone(),
        refresh_interval_secs: config.refresh_interval_secs,
        last_updated: now,
        cache_bust_url: format!("{}?ts={}", config.webcam_url, timestamp),
    };

    HttpResponse::Ok().json(info)
}

#[derive(serde::Serialize)]
pub struct WebcamUrlResponse {
    pub url: String,
    pub timestamp: i64,
}

pub async fn get_webcam_url(config: web::Data<Config>) -> HttpResponse {
    let timestamp = Utc::now().timestamp_millis();

    let response = WebcamUrlResponse {
        url: format!("{}?ts={}", config.webcam_url, timestamp),
        timestamp,
    };

    HttpResponse::Ok().json(response)
}

pub async fn proxy_webcam_image(config: web::Data<Config>) -> HttpResponse {
    let timestamp = Utc::now().timestamp_millis();
    let url = format!("{}?ts={}", config.webcam_url, timestamp);

    info!("Proxying webcam image from: {}", url);

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
    {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create HTTP client: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse::new(
                "client_error",
                "Failed to create HTTP client",
            ));
        }
    };

    match client.get(&url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                error!("Upstream returned error status: {}", response.status());
                return HttpResponse::BadGateway().json(ErrorResponse::new(
                    "upstream_error",
                    &format!("Upstream returned status: {}", response.status()),
                ));
            }

            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("image/jpeg")
                .to_string();

            match response.bytes().await {
                Ok(bytes) => HttpResponse::Ok()
                    .content_type(content_type)
                    .insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
                    .insert_header(("Pragma", "no-cache"))
                    .insert_header(("Expires", "0"))
                    .body(bytes),
                Err(e) => {
                    error!("Failed to read response body: {}", e);
                    HttpResponse::BadGateway().json(ErrorResponse::new(
                        "read_error",
                        "Failed to read upstream response",
                    ))
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch webcam image: {}", e);
            HttpResponse::BadGateway().json(ErrorResponse::new(
                "fetch_error",
                &format!("Failed to fetch webcam image: {}", e),
            ))
        }
    }
}

