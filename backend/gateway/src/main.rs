use shared::utils;
use tracing::info;

mod grpc_clients;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logging();

    // let user_addr = env::var("USER_GRPC")?;
    // let order_addr = env::var("ORDER_GRPC")?;
    // let product_addr = env::var("PRODUCT_GRPC")?;

    // let app_state = AppState::new(user_addr, order_addr, product_addr).await?;

    let app_state = grpc_clients::AppState::new("http://[::1]:50051".into()).await?;
    let app = routes::app_routes(app_state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("HTTP Gateway listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(utils::graceful_shutdown())
        .await?;

    Ok(())
}
