use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{handlers::*, state};

pub fn create_router(state: Arc<state::AppState>) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(state)
}
