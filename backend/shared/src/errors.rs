use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("gRPC error: {0}")]
    Grpc(String),
    #[error("Not found: {0}")]
    NotFound(String),
}
