use crate::errors::AppError;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub type DbPool = Arc<PgPool>;

pub async fn init_pg_pool<S: Into<String>>(
    dsn: S,
    max_connections: u32,
) -> Result<DbPool, AppError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .idle_timeout(Some(Duration::from_secs(20)))
        .acquire_timeout(Duration::from_secs(5))
        .connect(&dsn.into())
        .await?;

    info!("Connected to DB, MAX connections: {}", max_connections);

    Ok(Arc::new(pool))
}
