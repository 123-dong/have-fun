use crate::handlers::*;
use proto::user::v1::GetRequest;

pub(crate) async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let mut client = state.clients.user.clone();
    let request = tonic::Request::new(GetRequest { id });

    match client.get(request).await {
        Ok(response) => {
            let user = response.into_inner();
            Json(json!({ "id": user.id, "name": user.name }))
        }
        Err(err) => Json(json!({ "error": err.to_string() })),
    }
}
