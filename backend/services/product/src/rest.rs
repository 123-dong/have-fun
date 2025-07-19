use crate::handler::AppState;
use axum::{
    Json, Router,
    extract::{Extension, Path},
    routing::get,
};
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
pub struct HealthResponse {
    pub message: String,
}

async fn health_check(
    Path(name): Path<String>,
    Extension(state): Extension<AppState>,
) -> Json<HealthResponse> {
    let msg = state
        .do_health_check(&name)
        .await
        .unwrap_or_else(|_| "Error".into());
    info!("Handled REST health_check for {}", name);
    Json(HealthResponse { message: msg })
}

pub fn create_rest_router(state: AppState) -> Router {
    Router::new()
        .route("/health/:name", get(health_check))
        .layer(Extension(state))
}
