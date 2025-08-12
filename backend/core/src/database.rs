use crate::errors::AppError;

pub type DbPool = sqlx::PgPool;

pub async fn init_pool(db_url: &str, max_conn: u32) -> Result<DbPool, AppError> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(db_url)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}
