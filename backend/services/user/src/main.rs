use proto::DESCRIPTOR_SET;
use proto::user::v1::user_service_server::UserServiceServer;
use shared::database;

use std::sync::Arc;
use tonic::transport::Server;
use tonic_reflection::server::Builder;
use tracing::info;

mod repository;
mod service_impl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().compact().pretty().init();

    let db_url = "postgres://admin:123@localhost:5432/demo_db";
    let pool = database::init_pg_pool(db_url, 5).await?;
    let shared_pool = Arc::new(pool);

    let repo = repository::UserRepo::new(shared_pool.clone());

    let addr = "[::1]:50051".parse()?;
    let user_svc = service_impl::UserSvc { repo };

    let reflection_svc = Builder::configure()
        .register_encoded_file_descriptor_set(DESCRIPTOR_SET)
        .build()?;

    info!("UserService listening on {}", addr);

    Server::builder()
        .add_service(reflection_svc)
        .add_service(UserServiceServer::new(user_svc))
        .serve_with_shutdown(addr, shared::utils::graceful_shutdown())
        .await?;

    Ok(())
}
