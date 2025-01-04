use actix_web::{get, web, HttpResponse};

use crate::state::State;

#[get("")]
async fn internal_health() -> HttpResponse {
    HttpResponse::Ok().body("API is healthy")
}

#[get("/database")]
async fn database_health(state: web::Data<State>) -> HttpResponse {
    let result = sqlx::query!("SELECT version()").fetch_one(&state.db).await;

    if let Err(err) = result {
        return HttpResponse::InternalServerError().body(format!("Database is unhealthy: {err}"));
    }

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
