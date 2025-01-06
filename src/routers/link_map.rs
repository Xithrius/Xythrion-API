use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    database::models::link_map_model::{DestinationType, LinkMapModel},
    routers::ApiError,
};

#[get("/channels/all")]
async fn get_all_link_map_channels(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let converters = LinkMapModel::get_all_channels(&**pool).await?;

    Ok(HttpResponse::Ok().json(converters))
}

#[get("/converters/all")]
async fn get_all_link_map_converters(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let converters = LinkMapModel::get_all_converters(&**pool).await?;

    Ok(HttpResponse::Ok().json(converters))
}

#[get("/channels/{channel_id}")]
async fn get_one_link_map_channel(
    pool: web::Data<PgPool>,
    channel_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let channel = LinkMapModel::get_one_channel(&**pool, *channel_id).await?;

    Ok(HttpResponse::Ok().json(channel))
}

#[get("/converters/{converter_id}")]
async fn get_one_link_map_converter(
    pool: web::Data<PgPool>,
    converter_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let converter = LinkMapModel::get_one_converter(&**pool, *converter_id).await?;

    Ok(HttpResponse::Ok().json(converter))
}

#[get("/servers/{discord_server_id}/channels")]
async fn get_server_link_map_channels(
    pool: web::Data<PgPool>,
    discord_server_id: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    let channels = LinkMapModel::get_server_channels(&**pool, *discord_server_id).await?;

    Ok(HttpResponse::Ok().json(channels))
}

#[get("/servers/{discord_server_id}/converters")]
async fn get_server_link_map_converters(
    pool: web::Data<PgPool>,
    discord_server_id: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    let converters = LinkMapModel::get_server_converters(&**pool, *discord_server_id).await?;

    Ok(HttpResponse::Ok().json(converters))
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LinkMapChannelCreate {
    server_id: i64,
    input_channel_id: i64,
    output_channel_id: i64,
}

#[post("/channels")]
async fn create_link_map_channel(
    pool: web::Data<PgPool>,
    link_map_channel: web::Json<LinkMapChannelCreate>,
) -> Result<HttpResponse, ApiError> {
    LinkMapModel::insert_channel(
        &**pool,
        link_map_channel.server_id,
        link_map_channel.input_channel_id,
        link_map_channel.output_channel_id,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct LinkMapConverterCreate {
    from_link: String,
    to_link: Option<String>,
    xpath: Option<String>,
}

#[post("/converters")]
async fn create_link_map_converter(
    pool: web::Data<PgPool>,
    link_map_converter: web::Json<LinkMapConverterCreate>,
) -> Result<HttpResponse, ApiError> {
    let (to_link, xpath) = (
        link_map_converter.to_link.clone(),
        link_map_converter.xpath.clone(),
    );
    if (to_link.is_some() && xpath.is_some()) || (to_link.is_none() && xpath.is_none()) {
        return Err(ApiError::InvalidInput(
            "The destination for the converter can only be a link XOR an xpath".to_string(),
        ));
    }

    LinkMapModel::insert_converter(
        &**pool,
        link_map_converter.from_link.clone(),
        match (to_link, xpath) {
            (Some(to), None) => DestinationType::Link(to),
            (None, Some(x)) => DestinationType::XPath(x),
            _ => panic!("Already checked to link XOR xpath, this branch should never be reached"),
        },
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

#[put("/channels/{channel_id}/converters/{converter_id}/enable")]
async fn enable_link_map_converter_for_channel(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (channel_id, converter_id) = path.into_inner();
    LinkMapModel::enable_converter_for_channel(&**pool, channel_id, converter_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[put("/channels/{channel_id}/converters/{converter_id}/disable")]
async fn disable_link_map_converter_for_channel(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (channel_id, converter_id) = path.into_inner();
    LinkMapModel::disable_converter_for_channel(&**pool, channel_id, converter_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/channels/{channel_id}")]
async fn remove_link_map_channel(
    pool: web::Data<PgPool>,
    channel_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    LinkMapModel::remove_channel(&**pool, *channel_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/converters/{converter_id}")]
async fn remove_link_map_converter(
    pool: web::Data<PgPool>,
    converter_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    LinkMapModel::remove_converter(&**pool, *converter_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/link_maps")
            .service(get_all_link_map_channels)
            .service(get_all_link_map_converters)
            .service(get_one_link_map_channel)
            .service(get_one_link_map_converter)
            .service(get_server_link_map_channels)
            .service(get_server_link_map_converters)
            .service(create_link_map_channel)
            .service(create_link_map_converter)
            .service(enable_link_map_converter_for_channel)
            .service(disable_link_map_converter_for_channel)
            .service(remove_link_map_channel)
            .service(remove_link_map_converter),
    );
}
