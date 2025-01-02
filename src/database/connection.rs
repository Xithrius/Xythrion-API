use sqlx::{migrate::MigrateDatabase, postgres::PgPool, Connection, PgConnection, Postgres};
use tracing::info;

pub async fn connect(database_url: &str) -> PgPool {
    info!("Initializing database connection");

    PgPool::connect(database_url)
        .await
        .expect("Could not connect to postgres database")
}

pub async fn check_for_migrations(database_url: &str) -> Result<(), sqlx::Error> {
    if !Postgres::database_exists(database_url).await.unwrap() {
        info!("Creating database...");
        Postgres::create_database(database_url).await.unwrap();
    }

    info!("Applying migrations...");

    let mut conn: PgConnection = PgConnection::connect(database_url).await.unwrap();
    sqlx::migrate!()
        .run(&mut conn)
        .await
        .expect("Error while running database migrations!");

    Ok(())
}
