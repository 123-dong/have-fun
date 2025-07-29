use std::sync::Arc;

use user_service::{config, database, routes, signal, state};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing()?;

    let cfg = config::Config::from_env().map_err(|e| format!("Config error: {}", e))?;
    let db = Arc::new(database::init_pool(&cfg.db.url, cfg.db.max_connections).await?);

    let state = Arc::new(state::AppState { db });
    state.ping_db().await?;

    let router = routes::create_router(state);
    let addr = &cfg.app.http_addr;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, router)
        .with_graceful_shutdown(signal::shutdown_signal())
        .await?;

    Ok(())
}

fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename(".env")
        .or_else(|_| dotenvy::from_filename("services/user-service/.env"))?;
    tracing_subscriber::fmt().compact().init();
    Ok(())
}
