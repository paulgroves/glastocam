use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, World!"
}

#[get("/health")]
async fn health_check() -> impl Responder {
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Glastocam API server...");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health_check)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
