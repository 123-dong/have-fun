use axum::Router;

mod client;
mod middleware;
mod routes;

#[tokio::main]
async fn main() {
    let user_client = client::user_client().await;

    let app = Router::new().nest("/users", routes::user_router(user_client));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("HTTP Gateway listening on 0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
