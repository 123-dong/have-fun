use std::sync::Arc;
use tracing::info;

mod controller;
mod grpc_clients;
mod routes;
use shared::{config, utils};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logging();
    let cfg = config::AppConfig::from_env_file("./gateway/.env");

    let app_state = grpc_clients::AppState::new(cfg.user_grpc);
    info!(
        "Initial Arc reference count: {}",
        Arc::strong_count(&app_state.clients)
    );

    let app = routes::app_routes(app_state.clone());
    info!(
        "Arc reference count after routing: {}",
        Arc::strong_count(&app_state.clients)
    );

    let listener = tokio::net::TcpListener::bind("[::]:3000").await?;
    info!("HTTP Gateway listening on [::]:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(utils::shutdown_signal())
        .await?;

    Ok(())
}
