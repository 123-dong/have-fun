mod product_routes;
mod user_routes;
pub(super) use crate::grpc_clients::AppState; // pub clients: Arc<GrpcClients>

use tower_http::cors::{Any, CorsLayer};

pub(crate) fn app_routes(state: AppState) -> axum::Router {
    axum::Router::new()
        .nest("/users", user_routes::user_routes(state.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}
