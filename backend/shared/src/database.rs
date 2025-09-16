use crate::errors::AppError;
use std::time::Duration;

pub type DbPool = sqlx::PgPool;

pub async fn init_pg_pool<S: AsRef<str>>(dsn: S, max_connections: u32) -> Result<DbPool, AppError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .idle_timeout(Some(Duration::from_secs(60)))
        .acquire_timeout(Duration::from_secs(5))
        .connect(dsn.as_ref())
        .await?;
    Ok(pool)
}
