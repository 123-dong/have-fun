use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let status = match self {
            ServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = self.to_string();
        (status, body).into_response()
    }
}
