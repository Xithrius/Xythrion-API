use tracing::info;

use crate::{config::Config, database};

#[derive(Clone)]
pub struct State {
    pub db: sqlx::Pool<sqlx::Postgres>,
}

impl State {
    pub async fn new(config: Config) -> Self {
        let db_pool = Self::init_db(config.db_url()).await;

        Self { db: db_pool }
    }

    async fn init_db(database_url: String) -> sqlx::Pool<sqlx::Postgres> {
        info!("Connecting to Postgres database");

        database::connection::check_for_migrations(&database_url)
            .await
            .expect("Something went wrong when running database migrations");
        database::connection::connect(&database_url).await
    }
}
