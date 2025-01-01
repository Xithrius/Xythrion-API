pub mod health;
pub mod not_found;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(health::config)
        .default_service(web::get().to(not_found::not_found));
}
