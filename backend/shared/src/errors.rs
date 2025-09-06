use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Dotenvy error: {0}")]
    Dotenvy(#[from] dotenvy::Error),

    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Missing env var: {0}")]
    MissingVar(String),

    #[error("Reflection error: {0}")]
    Reflection(#[from] tonic_reflection::server::Error),

    #[error("Tonic error: {0}")]
    Tonic(#[from] tonic::transport::Error),
}
