mod auth;
mod product;
mod user;

pub(super) use crate::state::AppState;
pub(super) use axum::{Router, routing::get};

pub(crate) fn app_routes(state: AppState) -> Router {
    Router::new().nest("/users", user::user_routes(state.clone()))
}
