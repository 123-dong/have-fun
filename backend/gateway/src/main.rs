use shared::utils;
use tracing::info;

mod grpc_clients;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logging();

    // let clients = grpc_clients::AppState::new().await;
    let clients = grpc_clie;

    let app = routes::app_routes(clients.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("HTTP Gateway listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(utils::graceful_shutdown())
        .await?;

    Ok(())
}
