use crate::handler::AppState;
use crate::{grpc, rest};
use axum::serve;
use tokio::net::TcpListener;
// use tokio::task;
use tonic::transport::Server as GrpcServer;
use tracing::info;

pub async fn run(app_state: AppState) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let grpc_addr = "[::1]:50051".parse()?;
    let rest_addr = "[::1]:3000";
    let rest_listener = TcpListener::bind(rest_addr).await?;

    let rest_router = rest::create_rest_router(app_state.clone());
    let grpc_service = grpc::create_grpc_server(app_state);

    info!("Starting REST on {}", rest_addr);
    info!("Starting gRPC on {}", grpc_addr);

    let rest_handle = tokio::spawn(async move {
        if let Err(e) = serve(rest_listener, rest_router.into_make_service()).await {
            eprintln!("REST server error: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    });

    let grpc_handle = tokio::spawn(async move {
        if let Err(e) = GrpcServer::builder()
            .add_service(grpc_service)
            .serve(grpc_addr)
            .await
        {
            eprintln!("gRPC server error: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    });

    rest_handle.await??;
    grpc_handle.await??;

    Ok(())
}
