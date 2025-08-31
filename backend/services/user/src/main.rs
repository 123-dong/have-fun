mod repository;
mod service;
mod service_impl;

use proto::v1::DESCRIPTOR_SET;
use proto::v1::user::user_service_server::UserServiceServer;
use shared::{config, database, init_reflection, utils};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logging();

    let cfg = config::AppConfig::from_env_file("./services/user/.env");
    println!("Loaded config: {:?}", cfg);

    let pool = database::init_pg_pool(&cfg.database_url, 5).await?;
    info!("Postgres pool initialized");

    let user_repo = repository::UserRepo::new(pool);
    info!("Repository initialized");

    let user_svc = service::UserSvc::new(user_repo);
    info!("Service initialized");

    let svc_impl = service_impl::SvcImpl::new(user_svc);
    info!("Service implementation initialized");

    let listener_v4 = TcpListener::bind(("0.0.0.0", cfg.service_port)).await?;
    info!("Listening on 0.0.0.0:{} (IPv4)", cfg.service_port);

    let listener_v6 = TcpListener::bind(("::", cfg.service_port)).await?;
    info!("Listening on [::]:{} (IPv6)", cfg.service_port);

    let refl = init_reflection!(DESCRIPTOR_SET)?;
    info!("Reflection service initialized");

    let shutdown = utils::shutdown_signal();
    tokio::try_join!(
        tonic::transport::Server::builder()
            .add_service(refl.clone())
            .add_service(UserServiceServer::new(svc_impl.clone()))
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener_v4), shutdown.notified()),
        tonic::transport::Server::builder()
            .add_service(refl)
            .add_service(UserServiceServer::new(svc_impl))
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener_v6), shutdown.notified())
    )?;

    Ok(())
}
