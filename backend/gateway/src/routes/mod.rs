mod product_routes;
mod user_routes;

pub(super) use crate::grpc_clients::AppState;
pub(super) use axum::Router;

pub(crate) fn app_routes(state: AppState) -> Router {
    Router::new().nest("/users", user_routes::user_routes(state))
}
