use std::env;

use crate::internal_error;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use sqlx::{sqlite::SqlitePoolOptions, Connection, Pool, Sqlite, SqliteConnection, SqlitePool};

pub mod models;
pub mod impls;

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Sqlite>);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    SqlitePool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = SqlitePool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub async fn get_db_connection() -> sqlx::Result<SqliteConnection> {
    SqliteConnection::connect(&env::var("DATABASE_URL").expect("")).await
}

pub async fn new_db_pool() -> sqlx::Result<Pool<Sqlite>> {
    SqlitePoolOptions::new()
        .max_connections(4)
        .connect("sqlite:file:db/main.db")
        .await
}
