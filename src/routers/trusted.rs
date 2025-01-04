use actix_web::{delete, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{database::models::trusted_model::TrustedModel, routers::ApiError};

#[derive(Deserialize, Serialize)]
pub struct TrustedUserItem {
    pub user_id: i64,
}

#[get("/")]
async fn get_all_trusted_users(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let trusted_users = TrustedModel::get_all(&**pool).await?;

    Ok(HttpResponse::Ok().json(trusted_users))
}

#[get("/{user_id}")]
async fn get_trusted_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    let trusted_user = TrustedModel::get_one(&**pool, *user_id).await?;

    Ok(trusted_user.map_or_else(
        || HttpResponse::NotFound().body(format!("User with ID '{user_id}' could not be found")),
        |user| HttpResponse::Ok().json(user),
    ))
}

#[post("/")]
async fn create_trusted_user(
    pool: web::Data<PgPool>,
    new_trusted_user: web::Path<TrustedUserItem>,
) -> Result<HttpResponse, ApiError> {
    let user = TrustedModel::get_one(&**pool, new_trusted_user.user_id).await?;

    if user.is_some() {
        return Ok(HttpResponse::Conflict().body(format!(
            "User with ID '{}' is already trusted",
            new_trusted_user.user_id
        )));
    }

    TrustedModel::insert(&**pool, new_trusted_user.user_id).await?;

    Ok(HttpResponse::NoContent().into())
}

#[delete("/{user_id}")]
async fn remove_trusted_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    let user = TrustedModel::get_one(&**pool, *user_id).await?;

    if user.is_none() {
        return Ok(
            HttpResponse::NotFound().body(format!("User with ID '{user_id}' could not be found"))
        );
    }

    TrustedModel::remove(&**pool, *user_id).await?;

    Ok(HttpResponse::NoContent().into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/trusted")
            .service(get_all_trusted_users)
            .service(get_trusted_user)
            .service(create_trusted_user)
            .service(remove_trusted_user),
    );
}
