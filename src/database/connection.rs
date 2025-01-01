use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Connection, PgConnection, Postgres};
use std::time::Duration;
use tracing::info;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    info!("Initializing database connection");
    let database_url = dotenvy::var("DATABASE_URL").expect("`DATABASE_URL` not in .env");
    let pool = PgPoolOptions::new()
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .connect(&database_url)
        .await?;

    Ok(pool)
}
pub async fn check_for_migrations() -> Result<(), sqlx::Error> {
    let uri = dotenvy::var("DATABASE_URL").expect("`DATABASE_URL` not in .env");
    let uri = uri.as_str();
    if !Postgres::database_exists(uri).await? {
        info!("Creating database...");
        Postgres::create_database(uri).await?;
    }

    info!("Applying migrations...");

    let mut conn: PgConnection = PgConnection::connect(uri).await?;
    sqlx::migrate!()
        .run(&mut conn)
        .await
        .expect("Error while running database migrations!");

    Ok(())
}
