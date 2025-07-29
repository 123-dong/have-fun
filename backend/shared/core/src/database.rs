use sqlx::{Error, PgPool};

pub async fn init_pool(url: &str, max_connections: u32) -> Result<PgPool, Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
}

pub async fn check_db(pool: &PgPool) -> Result<(), Error> {
    sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
}
