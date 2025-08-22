// use axum::{Json, Router, extract::State, routing::post};
// use proto::user::v1::{
//     CreateRequest, GetRequest, UpdateRequest, user_service_client::UserServiceClient,
// };
// use serde::{Deserialize, Serialize};
// use tonic::transport::Channel;

// #[derive(Deserialize)]
// pub struct CreateUserPayload {
//     pub name: String,
//     pub email: String,
// }
// #[derive(Deserialize)]
// pub struct UpdateUserPayload {
//     pub id: String,
//     pub name: String,
//     pub email: String,
// }
// #[derive(Deserialize)]
// pub struct GetUserPayload {
//     pub id: String,
// }
// #[derive(Deserialize)]
// pub struct DeleteUserPayload {
//     pub id: String,
// }

// #[derive(Serialize)]
// pub struct UserResp {
//     pub id: String,
//     pub name: String,
//     pub email: String,
// }
// #[derive(Serialize)]
// pub struct DeleteResp {
//     pub success: bool,
// }

// // --- Handlers ---
// pub async fn create_user(
//     State(client): State<UserServiceClient<Channel>>,
//     Json(payload): Json<CreateUserPayload>,
// ) -> Json<UserResp> {
//     let req = CreateRequest {
//         name: payload.name,
//         email: payload.email,
//     };
//     let resp = client.clone().create(req).await.unwrap().into_inner();
//     Json(UserResp {
//         id: resp.id,
//         name: resp.name,
//         email: resp.email,
//     })
// }

// pub async fn get_user(
//     State(client): State<UserServiceClient<Channel>>,
//     Json(payload): Json<GetUserPayload>,
// ) -> Json<UserResp> {
//     let req = GetRequest { id: payload.id };
//     let resp = client.clone().get(req).await.unwrap().into_inner();
//     Json(UserResp {
//         id: resp.id,
//         name: resp.name,
//         email: resp.email,
//     })
// }

// pub async fn update_user(
//     State(client): State<UserServiceClient<Channel>>,
//     Json(payload): Json<UpdateUserPayload>,
// ) -> Json<UserResp> {
//     let req = UpdateRequest {
//         id: payload.id,
//         name: payload.name,
//         email: payload.email,
//     };
//     let resp = client.clone().update(req).await.unwrap().into_inner();
//     Json(UserResp {
//         id: resp.id,
//         name: resp.name,
//         email: resp.email,
//     })
// }

// pub async fn delete_user(
//     State(client): State<UserServiceClient<Channel>>,
//     Json(payload): Json<DeleteUserPayload>,
// ) -> Json<DeleteResp> {
//     let req = GetRequest { id: payload.id };
//     let resp = client.clone().delete(req).await.unwrap().into_inner();
//     Json(DeleteResp {
//         success: resp.success,
//     })
// }

// // --- Router ---
// pub fn router(client: UserServiceClient<Channel>) -> Router {
//     Router::new()
//         .route("/users/create", post(create_user))
//         .route("/users/get", post(get_user))
//         .route("/users/update", post(update_user))
//         .route("/users/delete", post(delete_user))
//         .with_state(client)
// }

use crate::dto::user::{CreateUserRequest, UserResponse};
use crate::grpc_client::UserClient;
use axum::{Json, Router, extract::State, routing::post};
use proto::user::v1::CreateRequest;

pub async fn create_user(
    State(client): State<UserClient>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<UserResponse> {
    let req = CreateRequest {
        name: payload.name,
        email: payload.email,
    };
    let resp = client.inner().create(req).await.unwrap().into_inner();

    Json(UserResponse {
        id: resp.id,
        name: resp.name,
        email: resp.email,
    })
}

pub fn router() -> Router<UserClient> {
    Router::new().route("/create", post(create_user))
}
