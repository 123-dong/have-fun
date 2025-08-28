use crate::grpc_clients::AppState; // pub clients: Arc<GrpcClients>
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use proto::v1::user::GetRequest;
use serde_json::json;

pub(crate) async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let clients = state.clients.clone();

    // gRPC request
    let req = tonic::Request::new(GetRequest { id });

    match clients.user.clone().get(req).await {
        Ok(resp) => {
            let user = resp.into_inner();
            Ok(Json(json!({
                "id": user.id,
                "name": user.name,
                "email": user.email,
            })))
        }
        Err(status) => {
            let code = match status.code() {
                tonic::Code::NotFound => StatusCode::NOT_FOUND,
                tonic::Code::InvalidArgument => StatusCode::BAD_REQUEST,
                _ => StatusCode::BAD_GATEWAY,
            };

            Err((
                code,
                Json(json!({
                    "error": status.code().to_string(),
                    "message": status.message(),
                })),
            ))
        }
    }
}
