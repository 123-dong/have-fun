use proto::DESCRIPTOR_SET;
use proto::user::v1::user_service_server::UserServiceServer;

mod repository;
mod service_impl;
use shared::database::*;

use tonic::transport::Server;
use tonic_reflection::server::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = "postgres://admin:123@localhost:5432/demo_db";
    let pool = init_pg_pool(db_url, 5).await?;
    init_db(&pool).await?;

    let addr = "[::1]:50051".parse()?;
    let user_service = service_impl::UserSvc { pool };

    let reflection = Builder::configure()
        .register_encoded_file_descriptor_set(DESCRIPTOR_SET)
        .build()?;

    println!("UserService listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .add_service(reflection)
        .serve_with_shutdown(addr, shared::utils::graceful_shutdown())
        .await?;

    Ok(())
}
