#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::struct_field_names,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

pub mod config;
pub mod cors;
pub mod database;
pub mod prometheus;
pub mod routers;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use config::Config;
use cors::default_cors;
use database::connection::init_database;
use prometheus::prometheus_middleware;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::new("config.toml".to_string());

    let database_url =
        std::env::var("XYTHRION_API_DATABASE_URL").unwrap_or_else(|_| config.db_url());

    let database_pool = init_database(database_url).await;

    let bind_address = config.bind_address();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(default_cors())
            .wrap(prometheus_middleware())
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(database_pool.clone()))
            .configure(routers::config)
    })
    .workers(2)
    .bind(bind_address)
    .expect("Failed to start Actix web service")
    .run()
    .await
}
