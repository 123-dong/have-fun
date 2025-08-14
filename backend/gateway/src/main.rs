mod grpc_client;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::app_router().await;
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
