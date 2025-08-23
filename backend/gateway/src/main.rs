use axum::Router;
use shared::utils;
use tracing::info;

mod dto;
mod grpc_client;
mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logging();

    let state = state::AppState::init().await;
    let router = routes::user::router();

    let app = Router::new()
        .nest("/users", router)
        .with_state(state.user.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("HTTP Gateway listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(utils::graceful_shutdown())
        .await?;

    Ok(())
}
