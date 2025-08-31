use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub service_port: u16,
}

impl AppConfig {
    pub fn from_env_file<P: AsRef<Path>>(env_path: P) -> Self {
        if env_path.as_ref().exists() {
            dotenvy::from_path(env_path).ok();
        } else {
            dotenvy::dotenv().ok(); // fallback
        }

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://admin:123@localhost:5432/demo_db".into());
        let service_port = std::env::var("SERVICE_PORT")
            .unwrap_or_else(|_| "50051".into())
            .parse()
            .expect("SERVICE_PORT must be a number");

        AppConfig {
            database_url,
            service_port,
        }
    }

    pub fn from_env() -> Self {
        Self::from_env_file(".env")
    }
}
