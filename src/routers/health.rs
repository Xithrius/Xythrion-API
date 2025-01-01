use actix_web::{get, web, HttpResponse};

#[get("")]
async fn internal_health() -> HttpResponse {
    HttpResponse::Ok().body("API is healthy")
}

#[get("/database")]
async fn database_health() -> HttpResponse {
    HttpResponse::Ok().body("Database is healthy")
}

#[get("/ping")]
async fn ping_pong() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ping_pong).service(
        web::scope("/health")
            .service(internal_health)
            .service(database_health),
    );
}
