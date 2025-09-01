mod repository;
mod service;
mod service_impl;

use proto::v1::DESCRIPTOR_SET;
use proto::v1::user::user_service_server::UserServiceServer;

use shared::{config, database, init_reflection, utils};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logging();

    let cfg = config::AppConfig::from_env_file("./services/user/.env");
    info!("{:?}", cfg);

    let pool = database::init_pg_pool(&cfg.database_url, 5).await?;

    let user_repo = repository::UserRepo::new(pool);
    let user_svc = service::UserSvc::new(user_repo);
    let svc_impl = service_impl::SvcImpl::new(user_svc);

    let refl = init_reflection!(DESCRIPTOR_SET)?;

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], cfg.service_port));

    tonic::transport::Server::builder()
        .add_service(refl)
        .add_service(UserServiceServer::new(svc_impl))
        .serve_with_shutdown(addr, utils::shutdown_signal())
        .await?;

    Ok(())
}
