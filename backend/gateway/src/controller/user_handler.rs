use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::json;
use tokio_stream::StreamExt;

use crate::grpc_clients::AppState; // pub clients: Arc<GrpcClients>
use proto::v1::user::{
    CreateRequest, DeleteRequest, GetRequest, ListBulkRequest, ListFullRequest, UpdateRequest,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let req = CreateRequest {
        name: payload["name"].as_str().unwrap_or_default().to_string(),
        email: payload["email"].as_str().unwrap_or_default().to_string(),
    };

    let resp = state
        .clients
        .user
        .clone()
        .create(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "user": resp.into_inner().user
    })))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let req = GetRequest { id };
    let resp = state
        .clients
        .user
        .clone()
        .get(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "user": resp.into_inner().user
    })))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let req = UpdateRequest {
        id,
        name: payload
            .get("name")
            .and_then(|v| v.as_str().map(|s| s.to_string())),
        email: payload
            .get("email")
            .and_then(|v| v.as_str().map(|s| s.to_string())),
    };

    let resp = state
        .clients
        .user
        .clone()
        .update(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "user": resp.into_inner().user
    })))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let req = DeleteRequest { id };
    let resp = state
        .clients
        .user
        .clone()
        .delete(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": resp.into_inner().success
    })))
}

pub async fn list_bulk(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let resp = state
        .clients
        .user
        .clone()
        .list_bulk(ListBulkRequest {})
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .into_inner();

    Ok(Json(json!({
        "users": resp.users
    })))
}

pub async fn list_full(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let mut stream = state
        .clients
        .user
        .clone()
        .list_full(ListFullRequest {})
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .into_inner();

    let mut users = vec![];
    while let Some(user_res) = stream.next().await {
        match user_res {
            Ok(u) => users.push(u),
            Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    Ok(Json(json!({
        "users": users
    })))
}
