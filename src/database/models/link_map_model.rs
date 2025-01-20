use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::DatabaseError;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct LinkMapChannel {
    id: Uuid,
    created_at: NaiveDateTime,
    server_id: i64,
    input_channel_id: i64,
    output_channel_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct LinkMapConverter {
    id: Uuid,
    created_at: NaiveDateTime,
    from_link: String,
    to_link: Option<String>,
    xpath: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct LinkMapServer {
    id: Uuid,
    created_at: NaiveDateTime,
    server_id: i64,
    input_channel_id: i64,
    output_channel_id: i64,
    converters: Vec<LinkMapConverter>,
}

pub enum DestinationType {
    Link(String),
    XPath(String),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LinkMapModel;

impl LinkMapModel {
    pub async fn insert_channel<'a, E>(
        exec: E,
        server_id: i64,
        input_channel_id: i64,
        output_channel_id: i64,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            "
            INSERT INTO link_map_channels (server_id, input_channel_id, output_channel_id)
            VALUES ($1, $2, $3)
            ",
            server_id,
            input_channel_id,
            output_channel_id
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn insert_converter<'a, E>(
        exec: E,
        source_link: String,
        destination: DestinationType,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let query = match destination {
            DestinationType::Link(to_link) => {
                sqlx::query!(
                    "
                    INSERT INTO link_map_converters (from_link, to_link)
                    VALUES ($1, $2)
                    ",
                    source_link,
                    to_link
                )
            }
            DestinationType::XPath(xpath) => {
                sqlx::query!(
                    "
                    INSERT INTO link_map_converters (from_link, xpath)
                    VALUES ($1, $2)
                    ",
                    source_link,
                    xpath
                )
            }
        };

        query.execute(exec).await?;

        Ok(())
    }

    pub async fn get_all_channels<'a, E>(exec: E) -> Result<Vec<LinkMapChannel>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let channels = sqlx::query!("SELECT * FROM link_map_channels",)
            .fetch_all(exec)
            .await?
            .into_iter()
            .map(|row| LinkMapChannel {
                id: row.id,
                created_at: row.created_at,
                server_id: row.server_id.expect("Server ID should be available"),
                input_channel_id: row
                    .input_channel_id
                    .expect("Input channel should be available"),
                output_channel_id: row
                    .output_channel_id
                    .expect("Output channel should be available"),
            })
            .collect::<Vec<LinkMapChannel>>();

        Ok(channels)
    }

    pub async fn get_all_converters<'a, E>(exec: E) -> Result<Vec<LinkMapConverter>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let converters = sqlx::query!("SELECT * FROM link_map_converters")
            .fetch_all(exec)
            .await?
            .into_iter()
            .map(|row| LinkMapConverter {
                id: row.id,
                created_at: row.created_at,
                from_link: row.from_link,
                to_link: row.to_link,
                xpath: row.xpath,
            })
            .collect::<Vec<LinkMapConverter>>();

        Ok(converters)
    }

    pub async fn get_one_channel<'a, E>(
        exec: E,
        channel_id: Uuid,
    ) -> Result<Option<LinkMapChannel>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let channel = sqlx::query!("SELECT * FROM link_map_channels WHERE id = $1", channel_id)
            .fetch_optional(exec)
            .await?
            .map(|row| LinkMapChannel {
                id: row.id,
                created_at: row.created_at,
                server_id: row.server_id.expect("Server ID should be available"),
                input_channel_id: row
                    .input_channel_id
                    .expect("Input channel should be available"),
                output_channel_id: row
                    .output_channel_id
                    .expect("Output channel should be available"),
            });

        Ok(channel)
    }

    pub async fn get_one_converter<'a, E>(
        exec: E,
        converter_id: Uuid,
    ) -> Result<Option<LinkMapConverter>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let converter = sqlx::query!(
            "SELECT * FROM link_map_converters WHERE id = $1",
            converter_id
        )
        .fetch_optional(exec)
        .await?
        .map(|row| LinkMapConverter {
            id: row.id,
            created_at: row.created_at,
            from_link: row.from_link,
            to_link: row.to_link,
            xpath: row.xpath,
        });

        Ok(converter)
    }

    pub async fn get_server_channels<'a, E>(
        exec: E,
        server_id: i64,
    ) -> Result<Vec<LinkMapChannel>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let server_channels = sqlx::query!(
            "SELECT * FROM link_map_channels WHERE server_id = $1",
            server_id
        )
        .fetch_all(exec)
        .await?
        .into_iter()
        .map(|row| LinkMapChannel {
            id: row.id,
            created_at: row.created_at,
            server_id,
            input_channel_id: row
                .input_channel_id
                .expect("Input channel could not be found"),
            output_channel_id: row
                .output_channel_id
                .expect("Output channel could not be found"),
        })
        .collect::<Vec<LinkMapChannel>>();

        Ok(server_channels)
    }

    pub async fn get_server_converters<'a, E>(
        exec: E,
        server_id: i64,
    ) -> Result<Vec<LinkMapServer>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let link_map_server = sqlx::query!(
            "
            SELECT
                lc.id AS channel_id,
                lc.created_at AS channel_created_at,
                lc.input_channel_id,
                lc.output_channel_id,
                c.id AS converter_id,
                c.created_at AS converter_created_at,
                c.from_link,
                c.to_link,
                c.xpath
            FROM link_map_channels lc
            JOIN channel_converter_association cca ON lc.id = cca.channel_id
            JOIN link_map_converters c ON cca.converter_id = c.id
            WHERE lc.server_id = $1;
            ",
            server_id
        )
        .fetch_all(exec)
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut acc, row| {
            let converter = LinkMapConverter {
                id: row.converter_id,
                created_at: row.converter_created_at,
                from_link: row.from_link,
                to_link: row.to_link,
                xpath: row.xpath,
            };

            acc.entry(row.channel_id)
                .or_insert_with(|| LinkMapServer {
                    id: row.channel_id,
                    created_at: row.channel_created_at,
                    server_id,
                    input_channel_id: row.input_channel_id.expect("Could not get input channel"),
                    output_channel_id: row.output_channel_id.expect("Could not get output channel"),
                    converters: Vec::new(),
                })
                .converters
                .push(converter);

            acc
        })
        .into_values()
        .collect::<Vec<LinkMapServer>>();

        Ok(link_map_server)
    }

    pub async fn enable_converter_for_channel<'a, E>(
        exec: E,
        channel_id: Uuid,
        converter_id: Uuid,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            "
            INSERT INTO channel_converter_association (channel_id, converter_id)
            VALUES ($1, $2)
            ",
            channel_id,
            converter_id
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn disable_converter_for_channel<'a, E>(
        exec: E,
        channel_id: Uuid,
        converter_id: Uuid,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            "
            DELETE FROM channel_converter_association WHERE channel_id = $1 AND converter_id = $2
            ",
            channel_id,
            converter_id
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn remove_channel<'a, E>(exec: E, id: Uuid) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!("DELETE FROM link_map_channels WHERE id = $1", id)
            .execute(exec)
            .await?;

        Ok(())
    }

    pub async fn remove_converter<'a, E>(exec: E, id: Uuid) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!("DELETE FROM link_map_converters WHERE id = $1", id)
            .execute(exec)
            .await?;

        Ok(())
    }
}
