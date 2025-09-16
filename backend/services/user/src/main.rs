mod repository;
mod service_impl;

use proto::v1::DESCRIPTOR_SET;
use proto::v1::user::user_service_server::UserServiceServer;
use shared::errors::AppError;
use shared::{config, database, utils};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    utils::init_logging();
    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    let cfg = config::AppConfig::from_env(env_path)?;
    tracing::info!("{:?}", cfg); // dev

    let pool = database::init_pg_pool(cfg.database.dsn, cfg.database.capacity).await?;
    let repo = repository::UserRepo::new(pool);
    let svc = service_impl::SvcImpl::new(repo);

    let reflection = utils::init_refl(DESCRIPTOR_SET)?;
    let addr = format!("[{}]:{}", cfg.web.web_host, cfg.web.web_port)
        .parse()
        .expect("Invalid host:port");

    tonic::transport::Server::builder()
        .add_service(reflection)
        .add_service(UserServiceServer::new(svc))
        .serve_with_shutdown(addr, utils::shutdown_signal())
        .await?;

    Ok(())
}
