// use axum::Json;
// use proto::user::v1::GetRequest;
// use proto::user::v1::user_service_client::UserServiceClient;
// use serde::Serialize;

// #[derive(Serialize)]
// pub(crate) struct GetUserResponse {
//     id: i32,
//     name: String,
// }

// pub async fn get_user_handler() -> Json<GetUserResponse> {
//     let mut client = UserServiceClient::connect("http://[::1]:50051")
//         .await
//         .expect("Failed to connect to gRPC service");

//     let request = tonic::Request::new(GetRequest { id: 1 });

//     let response = client
//         .get(request)
//         .await
//         .expect("gRPC request failed")
//         .into_inner();

//     Json(GetUserResponse {
//         id: response.id,
//         name: response.name,
//     })
// }

use crate::grpc_client::UserGrpcClient;
use axum::Json;
use shared::models;

pub async fn get_user_handler() -> Json<models::GetUserResponse> {
    let mut grpc = UserGrpcClient::connect("http://[::1]:50051").await;
    let resp = grpc.get_user(1).await;
    Json(models::GetUserResponse::from(resp))
}
