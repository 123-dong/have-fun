use crate::handlers::user::get_user;
use crate::routes::*;

pub(super) fn user_routes(state: AppState) -> Router {
    Router::new().route("/:id", get(get_user)).with_state(state)
}
