use thiserror::Error;
use tracing::error;

/// app error handler
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Dotenv error: {0}")]
    Dotenv(#[from] dotenvy::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Missing env var: {0}")]
    MissingVar(String),

    #[error("Reflection error: {0}")]
    Reflection(#[from] tonic_reflection::server::Error),

    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Unexpected error: {0}")]
    Other(String),

    #[error("Unexpected error: {0}")]
    Conflict(String),

    #[error("User not found")]
    NotFound,
}

/// service(gRPC) error handler
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Invalid UUID")]
    InvalidUuid,

    #[error("Not Found")]
    NotFound,

    #[error("Internal error: {0}")]
    Internal(String),
}

/// convert ServiceError -> tonic::Status
impl From<ServiceError> for tonic::Status {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::InvalidUuid => tonic::Status::invalid_argument(err.to_string()),
            ServiceError::NotFound => tonic::Status::not_found(err.to_string()),
            ServiceError::Internal(e) => {
                error!(%e, "internal service error");
                // Gửi thông báo chung, không gửi e
                tonic::Status::internal("internal server error")
            }
        }
    }
}

/// map AppError -> ServiceError.
impl From<AppError> for ServiceError {
    fn from(err: AppError) -> Self {
        if !matches!(err, AppError::NotFound) {
            error!(error = %err, "App error mapped to ServiceError");
        }
        match err {
            AppError::NotFound => ServiceError::NotFound,
            other => ServiceError::Internal(other.to_string()),
        }
    }
}
