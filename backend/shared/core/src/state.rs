use sqlx::{PgPool, error};
use std::sync::Arc;
use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}

impl AppState {
    pub async fn ping_db(&self) -> Result<(), error::Error> {
        sqlx::query("SELECT 1")
            .execute(&*self.db)
            .await
            .map(|_| info!("Ping OK"))
            .map_err(|e| {
                error!("Ping failed: {}", e);
                e
            })
    }
}
