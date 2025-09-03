use crate::controller::user_handler::*;
use crate::routes::*;
use axum::routing::{get, post};

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/full", get(list_full))
        .route("/bulk", get(list_bulk))
        .with_state(state)
}
