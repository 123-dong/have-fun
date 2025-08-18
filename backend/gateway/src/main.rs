mod client;
mod middleware;
mod router;

#[tokio::main]
async fn main() {
    let app = router::app_router().await;
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
