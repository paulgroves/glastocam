mod health;
mod webcam;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/v1")
                    .route("/health", web::get().to(health::health_check))
                    .route("/info", web::get().to(health::api_info))
                    .route("/webcam", web::get().to(webcam::get_webcam_info))
                    .route("/webcam/url", web::get().to(webcam::get_webcam_url))
                    .route("/webcam/proxy", web::get().to(webcam::proxy_webcam_image)),
            ),
    );
}

