use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub type DbPool = Arc<PgPool>;

pub async fn init_pg_pool(db_url: &str, max_conn: u32) -> Result<DbPool, sqlx::Error> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_conn)
        .idle_timeout(Some(Duration::from_secs(20)))
        .acquire_timeout(Duration::from_secs(5))
        .connect(db_url)
        .await?;

    info!("Connected to database, MAX connections: {}", max_conn);

    Ok(Arc::new(pool))
}

pub async fn ping_db(pool: &DbPool) -> sqlx::Result<()> {
    sqlx::query("SELECT 1").execute(pool.as_ref()).await?;
    info!("Ping database @K!");
    Ok(())
}
