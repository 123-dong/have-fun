mod repository;
mod service;
mod service_impl;

use proto::v1::DESCRIPTOR_SET;
use proto::v1::user::user_service_server::UserServiceServer;
use shared::{database, init_reflection, utils};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logging();

    let db_url = "postgres://admin:123@localhost:5432/demo_db";
    let pool = database::init_pg_pool(db_url, 5).await?;

    // repository -> service -> handler
    let user_repo = repository::UserRepo::new(pool);
    let user_svc = service::UserSvc::new(user_repo);
    let user_hdl = service_impl::SvcImpl::new(user_svc);

    let addr = "[::1]:50051".parse()?;
    info!("gRPC server listening on {}", addr);

    let refl = init_reflection!(DESCRIPTOR_SET)?;

    tonic::transport::Server::builder()
        .add_service(refl)
        .add_service(UserServiceServer::new(user_hdl))
        .serve_with_shutdown(addr, utils::graceful_shutdown())
        .await?;

    Ok(())
}
