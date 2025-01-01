#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod config;
mod cors;
mod database;
mod routers;

use actix_web::{middleware::Logger, App, HttpServer};

use cors::default_cors;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let host = "127.0.0.1";
    let port = 8080;

    database::connection::check_for_migrations()
        .await
        .expect("An error occurred while running migrations.");

    let pool = database::connection::connect()
        .await
        .expect("Database connection failed");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(default_cors())
            .configure(routers::config)
    })
    .workers(2)
    .bind((host, port))
    .expect("Failed to start Actix web service")
    .run()
    .await
}
