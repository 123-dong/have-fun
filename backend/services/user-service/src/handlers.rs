use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use tracing::{error, info};

use crate::{db::*, models::*, state::AppState};

pub async fn health_check(State(state): State<Arc<AppState>>) -> StatusCode {
    match ping_database(&state.database).await {
        Ok(_) => {
            info!("Health check OK");
            StatusCode::OK
        }
        Err(e) => {
            error!(error = ?e, "Health check failed");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    info!("Creating user");

    match insert_user(&state.database, &new_user).await {
        Ok(user) => {
            info!(user_id = %user.id, "User created");
            Ok(Json(user))
        }
        Err(e) => {
            error!(error = ?e, "Create failed");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<User>, StatusCode> {
    info!(user_id = %id, "Getting user");

    match get_user_by_id(&state.database, id).await {
        Ok(user) => Ok(Json(user)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!(user_id = %id, error = ?e, "Get failed");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    info!(user_id = %id, "Updating user");

    match update_user_by_id(&state.database, id, &payload).await {
        Ok(user) => {
            info!(user_id = %user.id, "User updated");
            Ok(Json(user))
        }
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!(user_id = %id, error = ?e, "Update failed");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<(), StatusCode> {
    info!(user_id = %id, "Deleting user");

    match delete_user_by_id(&state.database, id).await {
        Ok(_) => {
            info!(user_id = %id, "User deleted");
            Ok(())
        }
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!(user_id = %id, error = ?e, "Delete failed");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
