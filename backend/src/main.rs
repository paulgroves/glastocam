use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

mod config;
mod handlers;
mod models;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::from_env();
    let bind_address = format!("{}:{}", config.host, config.port);

    info!("Starting Glastocam API server at http://{}", bind_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(config.clone()))
            .configure(handlers::configure_routes)
            .service(Files::new("/", "../").index_file("index.html"))
    })
    .bind(&bind_address)?
    .run()
    .await
}

