use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct Postgres {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct Web {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database: Postgres,
    pub web: Web,
}

impl AppConfig {
    pub fn from_env(env_path: std::path::PathBuf) -> Result<Self, AppError> {
        dotenvy::from_path(env_path).map_err(|e| AppError::Dotenvy(e))?;

        let url = get_var("DATABASE_URL")?;
        let max_connections = get_var("MAX_CONNECTIONS")?.parse::<u32>()?;
        let host = get_var("WEB_HOST")?;
        let port = get_var("WEB_PORT")?;
        Ok(Self {
            database: Postgres {
                url,
                max_connections,
            },
            web: Web { host, port },
        })
    }
}

fn get_var(key: &str) -> Result<String, AppError> {
    dotenvy::var(key).map_err(|_| AppError::MissingVar(key.into()))
}
