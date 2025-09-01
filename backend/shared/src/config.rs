use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub service_port: u16,
    pub user_grpc: String,
    pub product_grpc: String,
}

impl AppConfig {
    pub fn from_env_file<P: AsRef<Path>>(env_path: P) -> Self {
        if dotenvy::from_path(env_path).is_err() {
            dotenvy::dotenv().ok(); // fallback
        }

        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://admin:123@localhost:5432/demo_db".into()),

            service_port: std::env::var("SERVICE_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),

            user_grpc: std::env::var("USER_GRPC").unwrap_or_else(|_| "http://[::1]:50058".into()),

            product_grpc: std::env::var("PRODUCT_GRPC")
                .unwrap_or_else(|_| "http://[::1]:50059".into()),
        }
    }
}
