#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod config;
mod cors;
mod database;
mod routers;
mod state;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use config::Config;
use cors::default_cors;
use state::State;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::new("config.toml".to_string());
    let state = State::new(config.clone()).await;

    let bind_address = config.bind_address();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(default_cors())
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(state.clone()))
            .configure(routers::config)
    })
    .workers(2)
    .bind(bind_address)
    .expect("Failed to start Actix web service")
    .run()
    .await
}
