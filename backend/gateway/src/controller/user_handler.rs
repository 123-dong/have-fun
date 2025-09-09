use axum::{
    Json,
    extract::{Path, State},
};

use axum::response::sse::{Event, Sse};
use std::convert::Infallible;
use tokio_stream::StreamExt;

// use crate::errors::AppError;
use crate::grpc_clients::AppState; // pub clients: Arc<GrpcClients>
use proto::v1::user::{
    CreateRequest, DeleteRequest, GetRequest, ListBulkRequest, ListFullRequest, UpdateRequest,
};
use proto::v1::user::{
    CreateResponse, DeleteResponse, GetResponse, ListBulkResponse, UpdateResponse,
};

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
) -> Result<Json<ListBulkResponse>, axum::http::StatusCode> {
    let resp = state
        .clients
        .user
        .clone()
        .list_bulk(ListBulkRequest {})
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(resp.into_inner()))
}

// pub async fn stream_user(
//     State(state): State<AppState>,
// ) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>>, axum::http::StatusCode>
// {
//     let mut client = state.clients.user.clone();

//     let mut stream = client
//         .list_full(ListFullRequest {})
//         .await
//         .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
//         .into_inner();

//     // channel map gRPC stream -> SSE stream
//     let (tx, rx) = tokio::sync::mpsc::channel(16);

//     // running task forward data
//     tokio::spawn(async move {
//         while let Some(user) = stream.next().await {
//             match user {
//                 Ok(user) => {
//                     let event = Event::default()
//                         .json_data(&user) // serialize User -> JSON
//                         .unwrap();
//                     let _ = tx.send(Ok(event)).await;
//                 }
//                 Err(_) => break,
//             }
//         }
//         // ended log
//         let _ = tx
//             .send(Ok(Event::default().event("done").data("stream ended")))
//             .await;
//     });

//     // wrap stream -> SSE
//     Ok(
//         Sse::new(tokio_stream::wrappers::ReceiverStream::new(rx)).keep_alive(
//             axum::response::sse::KeepAlive::new()
//                 .interval(tokio::time::Duration::from_secs(10))
//                 .text("keep-alive"),
//         ),
//     )
// }

// pub async fn stream_user(
//     State(state): State<AppState>,
// ) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>>, axum::http::StatusCode>
// {
//     let mut stream = state
//         .clients
//         .user
//         .clone()
//         .list_full(ListFullRequest {})
//         .await
//         .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
//         .into_inner();

//     let (tx, rx) = tokio::sync::mpsc::channel(8);

//     tokio::spawn(async move {
//         while let Some(user) = stream.next().await {
//             if let Ok(user) = user {
//                 if let Ok(event) = Event::default().json_data(&user) {
//                     let _ = tx.send(Ok(event)).await;
//                 }
//             }
//         }
//     });

//     Ok(Sse::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
// }

pub async fn stream_user(
    State(state): State<AppState>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>>, axum::http::StatusCode>
{
    let mut stream = state
        .clients
        .user
        .clone()
        .list_full(ListFullRequest {})
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .into_inner();

    // map gRPC stream -> SSE stream
    let sse_stream = async_stream::stream! {
        while let Some(u) = stream.next().await {
            match u {
                Ok(user) => {
                    if let Ok(event) = Event::default().json_data(&user) {
                        yield Ok(event);
                    }
                }
                Err(_) => break, // fast panic
            }
        }

        yield Ok(Event::default().event("done").data("stream ended"));
    };

    Ok(Sse::new(sse_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(tokio::time::Duration::from_secs(10))
            .text("keep-alive"),
    ))
}
