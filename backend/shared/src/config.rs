use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub database_url: String,
    pub grpc_port: u16,
    pub rest_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Config::from_env_with_defaults(50051, 8080).unwrap()
    }

    pub fn from_env_with_defaults(
        default_grpc_port: u16,
        default_rest_port: u16,
    ) -> Result<Self, String> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let grpc_port = match env::var("GRPC_PORT") {
            Ok(port_str) => port_str
                .parse()
                .map_err(|e| format!("Invalid GRPC_PORT: {}", e)),
            Err(_) => Ok(default_grpc_port),
        };

        let rest_port = match env::var("REST_PORT") {
            Ok(port_str) => port_str
                .parse()
                .map_err(|e| format!("Invalid REST_PORT: {}", e)),
            Err(_) => Ok(default_rest_port),
        };

        let grpc_port = grpc_port?;
        let rest_port = rest_port?;

        Ok(Self {
            database_url,
            grpc_port,
            rest_port,
        })
    }
}
