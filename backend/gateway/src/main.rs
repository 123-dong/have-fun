use shared::utils;
use tracing::info;

mod grpc_client;
mod handlers;
mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logging();

    let clients = grpc_client::GrpcClients::new().await;
    let state = state::AppState { clients };
    let app = routes::app_routes(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("HTTP Gateway listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(utils::graceful_shutdown())
        .await?;

    Ok(())
}
