use std::env;
use tracing::{error, info, warn};

const DEFAULT_HTTP_ADDR: &str = "0.0.0.0:3000";
const DEFAULT_GRPC_ADDR: &str = "0.0.0.0:50051";
const DEFAULT_MAX_CONNECTIONS: u32 = 5;

pub struct Config {
    pub app: AppConfig,
    pub db: DatabaseConfig,
}

pub struct AppConfig {
    pub http_addr: String,
    pub grpc_addr: String,
}

pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let http_addr = env::var("HTTP_ADDR").unwrap_or_else(|_| {
            warn!("HTTP_ADDR not set, using default: {}", DEFAULT_HTTP_ADDR);
            DEFAULT_HTTP_ADDR.to_string()
        });
        let grpc_addr = env::var("GRPC_ADDR").unwrap_or_else(|_| {
            warn!("GRPC_ADDR not set, using default: {}", DEFAULT_GRPC_ADDR);
            DEFAULT_GRPC_ADDR.to_string()
        });
        info!("HTTP_ADDR: {}, GRPC_ADDR: {}", http_addr, grpc_addr);

        let url = env::var("DATABASE_URL").map_err(|_| {
            error!("DATABASE_URL not set");
            "DATABASE_URL must be set".to_string()
        })?;
        if !url.starts_with("postgres://") {
            error!("Invalid DATABASE_URL");
            return Err("DATABASE_URL must start with 'postgres://'".to_string());
        }

        let max_connections = env::var("MAX_CONNECTIONS")
            .unwrap_or_default()
            .parse()
            .unwrap_or_else(|_| {
                warn!(
                    "Invalid MAX_CONNECTIONS, using default: {}",
                    DEFAULT_MAX_CONNECTIONS
                );
                DEFAULT_MAX_CONNECTIONS
            });
        info!("MAX_CONNECTIONS: {}", max_connections);

        Ok(Self {
            app: AppConfig {
                http_addr,
                grpc_addr,
            },
            db: DatabaseConfig {
                url,
                max_connections,
            },
        })
    }
}

impl AppConfig {
    pub fn parse_http_addr(&self) -> Result<std::net::SocketAddr, std::net::AddrParseError> {
        self.http_addr.parse()
    }
}
