use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamInfo {
    pub id: u32,
    pub name: String,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message.to_string()),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

pub struct AppState {
    pub cameras: Mutex<Vec<CamInfo>>,
    pub refresh_interval_ms: Mutex<u64>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            cameras: Mutex::new(vec![CamInfo {
                id: 1,
                name: "Glasto Main".to_string(),
                url: "https://panodata.panomax.com/cams/879/recent_full.jpg".to_string(),
                description: "Main Glastonbury Festival webcam feed".to_string(),
            }]),
            refresh_interval_ms: Mutex::new(300000), // 5 minutes
        }
    }
}

#[get("/api/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "status": "healthy",
        "service": "glastocam-api"
    })))
}

#[get("/api/cameras")]
async fn get_cameras(data: web::Data<AppState>) -> impl Responder {
    let cameras = data.cameras.lock().unwrap();
    HttpResponse::Ok().json(ApiResponse::success(cameras.clone()))
}

#[get("/api/cameras/{id}")]
async fn get_camera(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let cameras = data.cameras.lock().unwrap();

    if let Some(camera) = cameras.iter().find(|c| c.id == id) {
        HttpResponse::Ok().json(ApiResponse::success(camera.clone()))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("Camera not found"))
    }
}

#[get("/api/cameras/{id}/url")]
async fn get_camera_url(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let cameras = data.cameras.lock().unwrap();

    if let Some(camera) = cameras.iter().find(|c| c.id == id) {
        let timestamp = Utc::now().timestamp_millis();
        let url_with_cache_bust = format!("{}?ts={}", camera.url, timestamp);
        HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
            "url": url_with_cache_bust,
            "cache_bust_timestamp": timestamp
        })))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("Camera not found"))
    }
}

#[derive(Serialize)]
struct SettingsResponse {
    refresh_interval_ms: u64,
}

#[get("/api/settings")]
async fn get_settings(data: web::Data<AppState>) -> impl Responder {
    let interval = *data.refresh_interval_ms.lock().unwrap();
    HttpResponse::Ok().json(ApiResponse::success(SettingsResponse {
        refresh_interval_ms: interval,
    }))
}

#[derive(Deserialize)]
struct UpdateSettingsRequest {
    refresh_interval_ms: Option<u64>,
}

async fn update_settings(
    body: web::Json<UpdateSettingsRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Some(interval) = body.refresh_interval_ms {
        if interval < 10000 {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "Refresh interval must be at least 10 seconds",
            ));
        }
        *data.refresh_interval_ms.lock().unwrap() = interval;
    }

    let current_interval = *data.refresh_interval_ms.lock().unwrap();
    HttpResponse::Ok().json(ApiResponse::success(SettingsResponse {
        refresh_interval_ms: current_interval,
    }))
}

#[get("/api/info")]
async fn get_info() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "name": "Glastocam API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Backend API for the Glastonbury Festival webcam viewer"
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let app_state = web::Data::new(AppState::default());
    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    log::info!(
        "Starting Glastocam API server at http://{}:{}",
        bind_address,
        port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(health_check)
            .service(get_cameras)
            .service(get_camera)
            .service(get_camera_url)
            .service(get_settings)
            .service(get_info)
            .route("/api/settings", web::put().to(update_settings))
            .service(Files::new("/", "../").index_file("index.html"))
    })
    .bind((bind_address, port))?
    .run()
    .await
}
