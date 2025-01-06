pub mod connection;
pub mod models;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database operation/interaction error: {0}")]
    Database(#[from] sqlx::Error),
}
