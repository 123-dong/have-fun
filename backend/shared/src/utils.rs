// use sqlx::{Error, PgPool};
// use std::sync::Arc;
use tracing::info;
// use uuid::Uuid;

pub async fn graceful_shutdown() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.unwrap();
    }

    info!("Shutdown signal received");
}

// pub fn generate_id() -> Uuid {
//     Uuid::new_v4()
// }

// pub async fn check_db(pool: &PgPool) -> Result<(), Error> {
//     sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
// }

// #[derive(Clone)]
// pub struct AppState {
//     pub db: Arc<PgPool>,
// }

// impl AppState {
//     pub async fn ping_db(&self) -> Result<(), Error> {
//         sqlx::query("SELECT 1")
//             .execute(&*self.db)
//             .await
//             .map(|_| info!("Ping OK"))
//             .map_err(|e| {
//                 error!("Ping failed: {}", e);
//                 e
//             })
//     }
// }
