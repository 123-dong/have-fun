use crate::handlers::get_user_handler;
use axum::{Router, routing::get};

pub async fn app_router() -> Router {
    Router::new().route("/user", get(get_user_handler))
}
