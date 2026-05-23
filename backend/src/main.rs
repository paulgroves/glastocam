use actix_web::{web, App, HttpServer, HttpResponse};
use serde_json::json;

async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "Hello, World!",
        "status": "success"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello_world))
            .route("/api/hello", web::get().to(hello_world))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

