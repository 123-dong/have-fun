use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_pg_pool(db_url: &str, max_conn: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(db_url)
        .await
}
