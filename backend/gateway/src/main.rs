// use std::sync::Arc;
use tracing::info;

mod controller;
mod grpc_clients;
mod routes;
use shared::{config, errors, utils};

#[tokio::main]
async fn main() -> Result<(), errors::AppError> {
    utils::init_logging();
    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    let cfg = config::AppConfig::from_env(env_path)?;
    info!("{:?}", cfg);

    let app_state = grpc_clients::AppState::new(cfg.svc.user_addr);
    let app = routes::app_routes(app_state.clone());

    let addr = format!("[{}]:{}", cfg.web.web_host, cfg.web.web_port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(utils::shutdown_signal())
        .await?;

    Ok(())
}
