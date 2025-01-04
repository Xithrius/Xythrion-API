use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::routers::ApiError;

#[get("/channels/all")]
async fn get_all_link_map_channels(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    // channels = await link_map_channel_dao.get_all(session)
    // return list(channels)
    todo!()
    // -> list[LinkMapChannelModel]:
}

#[get("/converters/all")]
async fn get_all_link_map_converters(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    // converters = await link_map_converter_dao.get_all(session)
    // return list(converters)
    todo!()
    // -> list[LinkMapConverterModel]
}

#[get("/channels/{channel_id}")]
async fn get_one_link_map_channel(
    pool: web::Data<PgPool>,
    channel_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    // channel = await link_map_channel_dao.select_by_id(session, pk=channel_id)
    // if channel is None:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"Link map channel with ID '{channel_id}' could not be found",
    //     )
    // return channel
    todo!()
    // -> LinkMapChannelModel
}

#[get("/converters/{converter_id}")]
async fn get_one_link_map_converter(
    pool: web::Data<PgPool>,
    converter_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    // converter = await link_map_converter_dao.select_by_id(session, pk=converter_id)
    // if converter is None:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"Link map converter with ID '{converter_id}' could not be found",
    //     )
    // return converter
    todo!()
    // -> LinkMapConverterModel
}

#[get("/server/{discord_server_id}/channels")]
async fn get_server_link_map_channels(
    pool: web::Data<PgPool>,
    discord_server_id: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    // channels = await link_map_channel_dao.get_by_server_id(
    //     session, server_id=discord_server_id
    // )
    // return list(channels)
    todo!()
    // -> list[LinkMapChannelModel]
}

#[get("/server/{discord_server_id}/converters")]
async fn get_server_link_map_converters(
    pool: web::Data<PgPool>,
    discord_server_id: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    // converters = await link_map_converter_dao.get_by_server_id(
    //     session, server_id=discord_server_id
    // )
    // return list(converters)
    todo!()
    // -> list[LinkMapConverterModel]
}

#[get("/channels/{discord_channel_id}/converters")]
async fn get_discord_channel_converters(
    pool: web::Data<PgPool>,
    discord_server_id: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    // items = await link_map_channel_dao.get_converters_for_channel(
    //     session,
    //     input_channel_id=discord_channel_id,
    // )
    // if (converters := items) is None:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"No link map converters for discord channel with ID {discord_channel_id} could be found",
    //     )
    // return converters
    todo!()
    // -> list[LinkMapConverterModel]
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
    // channels = await link_map_channel_dao.get_by_server_id(
    //     session,
    //     server_id=link_map_channel.server_id,
    // )
    // if channels:
    //     raise HTTPException(
    //         status_code=status.HTTP_409_CONFLICT,
    //         detail=f"Link map channel already exists in server with ID '{link_map_channel.server_id}'",
    //     )
    // return await link_map_channel_dao.create(session, obj=link_map_channel)
    todo!()
    // -> LinkMapChannelModel
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
    // if (link_map_converter.to_link is None) == (link_map_converter.xpath is None):
    //     raise HTTPException(
    //         status_code=status.HTTP_400_BAD_REQUEST,
    //         detail="Only populate `to_link` or `xpath`.",
    //     )
    // converter = await link_map_converter_dao.get_by_link(
    //     session, link_map=link_map_converter
    // )
    // if converter is not None:
    //     raise HTTPException(
    //         status_code=status.HTTP_409_CONFLICT,
    //         detail="Link map converter already exists",
    //     )
    // return await link_map_converter_dao.create(session, obj=link_map_converter)
    todo!()
    // -> LinkMapConverterModel
}

#[put("/channels/{channel_id}/converters/{converter_id}/enable")]
async fn enable_link_map_converter_for_channel(
    pool: web::Data<PgPool>,
    channel_id: web::Path<Uuid>,
    converter_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    // # Make sure the converter exists
    // channel = await link_map_channel_dao.select_by_id(session, pk=channel_id)
    // if channel is None:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"Link map channel with ID '{channel_id}' could not be found",
    //     )
    // # Make sure the channel exists
    // converter = await link_map_converter_dao.select_by_id(session, pk=converter_id)
    // if converter is None:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"Link map converter with ID '{converter_id}' could not be found",
    //     )
    // # Add the converter to the channel
    // modified_converter_id = await link_map_channel_dao.add_converter(
    //     session,
    //     channel_id=channel_id,
    //     converter_id=converter_id,
    // )
    // if modified_converter_id is None:
    //     raise HTTPException(
    //         status_code=status.HTTP_409_CONFLICT,
    //         detail=f"Link map channel '{channel_id}' already has converter '{converter_id}' enabled",
    //     )
    // return Response(status_code=status.HTTP_204_NO_CONTENT)
    todo!()
    // -> Response
}

#[put("/channels/{channel_id}/converters/{converter_id}/disable")]
async fn disable_link_map_converter_for_channel(
    pool: web::Data<PgPool>,
    channel_id: web::Path<Uuid>,
    converter_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    // # Make sure the converter exists
    // channel = await link_map_channel_dao.select_by_id(session, pk=channel_id)
    // if channel is None:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"Link map channel with ID '{channel_id}' could not be found",
    //     )
    // # Make sure the channel exists
    // converter = await link_map_converter_dao.select_by_id(session, pk=converter_id)
    // if converter is None:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"Link map converter with ID '{converter_id}' could not be found",
    //     )
    // # Remove the converter from the channel
    // await link_map_channel_dao.remove_converter(
    //     session,
    //     channel_id=channel_id,
    //     converter_id=converter_id,
    // )
    // return Response(status_code=status.HTTP_204_NO_CONTENT)
    todo!()
    // -> Response
}

#[delete("/channels/{channel_id}")]
async fn remove_link_map_channel(
    pool: web::Data<PgPool>,
    channel_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    // count = await link_map_channel_dao.delete(session, pk=channel_id, cascade_once=True)
    // if count == 0:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"Link map channel with ID '{channel_id}' does not exist.",
    //     )
    // return Response(status_code=status.HTTP_204_NO_CONTENT)
    todo!()
    // -> Response
}

#[delete("/converters/{converter_id}")]
async fn remove_link_map_converter(
    pool: web::Data<PgPool>,
    converter_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    // count = await link_map_converter_dao.delete(
    //     session, pk=converter_id, cascade_once=True
    // )
    // if count == 0:
    //     raise HTTPException(
    //         status_code=status.HTTP_404_NOT_FOUND,
    //         detail=f"Link map converter with ID '{converter_id}' does not exist.",
    //     )
    // return Response(status_code=status.HTTP_204_NO_CONTENT)
    todo!()
    // -> Response
}
