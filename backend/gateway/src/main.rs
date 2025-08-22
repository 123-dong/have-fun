use axum::Router;
use shared::{init_logging, utils};
use tracing::info;

// mod client;
mod dto;
mod grpc_client;
// mod middleware;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    init_logging!();
    let state = state::AppState::init().await;

    // let user_client = client::user_client().await;
    // let app = Router::new().nest("/users", routes::user_router(user_client));

    let router = routes::user::router();

    let app = Router::new()
        .nest("/users", router)
        .with_state(state.user.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("HTTP Gateway listening on 0.0.0.0:3000");

    // axum::serve(listener, app).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(utils::graceful_shutdown())
        .await
        .unwrap();
}
