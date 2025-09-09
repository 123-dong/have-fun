use crate::controller::user_handler::*;
use crate::routes::*;
use axum::routing::get;

pub fn user_routes(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/", get(list_user).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/stream", get(stream_user))
        .with_state(state.clone())
}
