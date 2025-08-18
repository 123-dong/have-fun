use axum::{Router, routing::get};

pub async fn app_router() -> Router {
    Router::new().route("/users", get())
}
