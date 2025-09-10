use thiserror::Error;

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

    #[error("User not found")]
    NotFound,
}
