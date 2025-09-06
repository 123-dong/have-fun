use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct Postgres {
    pub dsn: String,
    pub capacity: u32,
}

#[derive(Debug, Clone)]
pub struct WebConfig {
    pub web_host: String,
    pub web_port: String,
}

#[derive(Debug, Clone)]
pub struct ServicesConfig {
    pub user_addr: String,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database: Postgres,
    pub web: WebConfig,
    pub svc: ServicesConfig,
}

impl AppConfig {
    pub fn from_env(env_path: std::path::PathBuf) -> Result<Self, AppError> {
        dotenvy::from_path(env_path)?;

        let db_url = get_var("DATABASE_URL")?;
        let max_connections = get_var("MAX_CONNECTIONS")?.parse::<u32>()?;
        let host = get_var("WEB_HOST")?;
        let port = get_var("WEB_PORT")?;
        let user_grpc = get_var("USER_GRPC")?;

        Ok(Self {
            database: Postgres {
                dsn: db_url,
                capacity: max_connections,
            },
            web: WebConfig {
                web_host: host,
                web_port: port,
            },
            svc: ServicesConfig {
                user_addr: user_grpc,
            },
        })
    }
}

fn get_var(key: &str) -> Result<String, AppError> {
    dotenvy::var(key).map_err(|_| AppError::MissingVar(key.into()))
}
