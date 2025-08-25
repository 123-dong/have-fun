use crate::handlers::*;
use proto::user::v1::GetRequest;

pub(crate) async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut client = state.clients.user.clone();
    let request = tonic::Request::new(GetRequest { id });

    match client.get(request).await {
        Ok(response) => {
            let user = response.into_inner();
            Ok(Json(json!({
                "id": user.id,
                "name": user.name,
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
