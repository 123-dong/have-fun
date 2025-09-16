use axum::{
    Json,
    extract::{Path, State},
};

use axum::response::sse::{Event, Sse};
use std::convert::Infallible;
use tokio_stream::StreamExt;

use crate::grpc_clients::AppState; // pub clients: Arc<GrpcClients>
use proto::v1::user::{
    CreateRequest, DeleteRequest, GetRequest, ListRequest, StreamRequest, UpdateRequest,
};
use proto::v1::user::{CreateResponse, DeleteResponse, GetResponse, ListResponse, UpdateResponse};
// use crate::errors::AppError;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> Result<Json<CreateResponse>, axum::http::StatusCode> {
    let req = tonic::Request::new(CreateRequest {
        name: payload.name,
        email: payload.email,
    });

    let resp = state
        .clients
        .user
        .clone()
        .create(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(resp.into_inner()))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetResponse>, axum::http::StatusCode> {
    let req = GetRequest { id };

    let resp = state
        .clients
        .user
        .clone()
        .get(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(resp.into_inner()))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateRequest>,
) -> Result<Json<UpdateResponse>, axum::http::StatusCode> {
    let req = UpdateRequest {
        id,
        name: payload.name,
        email: payload.email,
    };

    let resp = state
        .clients
        .user
        .clone()
        .update(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(resp.into_inner()))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DeleteResponse>, axum::http::StatusCode> {
    let req = DeleteRequest { id };
    let resp = state
        .clients
        .user
        .clone()
        .delete(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(resp.into_inner()))
}

pub async fn list_user(
    State(state): State<AppState>,
) -> Result<Json<ListResponse>, axum::http::StatusCode> {
    let resp = state
        .clients
        .user
        .clone()
        .list(ListRequest {})
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(resp.into_inner()))
}

pub async fn stream_user(
    State(state): State<AppState>,
) -> Result<
    Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>> + Send + 'static>,
    axum::http::StatusCode,
> {
    let mut stream = state
        .clients
        .user
        .clone()
        .stream(StreamRequest {})
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .into_inner();

    let sse_stream = async_stream::stream! {
        while let Some(user) = stream.next().await {
            match user {
                Ok(u) => {
                    if let Ok(event) = Event::default().json_data(&u) {
                        yield Ok(event);
                    }
                }
                Err(e) => {
                    eprintln!("gRPC stream error: {:?}", e);
                    break;
                }
            }
        }
    };

    Ok(Sse::new(Box::pin(sse_stream)))
}
