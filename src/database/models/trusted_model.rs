use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::DatabaseError;

#[derive(Deserialize, Serialize, Clone)]
pub struct TrustedModel {
    pub id: Uuid,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
}

impl TrustedModel {
    pub async fn insert<'a, E>(exec: E, user_id: i64) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!("INSERT INTO trusted (user_id) VALUES ($1)", user_id)
            .execute(exec)
            .await?;

        Ok(())
    }

    pub async fn get_all<'a, E>(exec: E) -> Result<Vec<Self>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let trusted_users = sqlx::query!("SELECT * FROM trusted")
            .fetch_all(exec)
            .await?
            .into_iter()
            .map(|row| Self {
                id: row.id,
                user_id: row.user_id,
                created_at: row.created_at,
            })
            .collect::<Vec<_>>();

        Ok(trusted_users)
    }

    pub async fn get_one<'a, E>(exec: E, user_id: i64) -> Result<Option<Self>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let trusted_user = sqlx::query!("SELECT * FROM trusted WHERE user_id = $1", user_id)
            .fetch_optional(exec)
            .await?
            .map(|row| Self {
                id: row.id,
                user_id: row.user_id,
                created_at: row.created_at,
            });

        Ok(trusted_user)
    }

    pub async fn remove<'a, E>(exec: E, user_id: i64) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!("DELETE FROM trusted WHERE (user_id = $1)", user_id)
            .execute(exec)
            .await?;

        Ok(())
    }
}
