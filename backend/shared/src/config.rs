use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub grpc_user_addr: String,
}

impl Config {
    pub fn load() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            grpc_user_addr: env::var("GRPC_USER_ADDR")?,
        })
    }
}
