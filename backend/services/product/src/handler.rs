use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

// Các hàm xử lý chung
impl AppState {
    pub async fn do_health_check(&self, name: &str) -> Result<String, sqlx::Error> {
        sqlx::query("INSERT INTO health_logs (name) VALUES ($1)")
            .bind(name)
            .execute(&self.db)
            .await?;

        Ok(format!("Running from {}!", name))
    }
}
