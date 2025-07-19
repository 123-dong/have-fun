use axum::{Router, routing::get};
use std::env;
use tokio::sync::oneshot;
use tonic::{Request, Response, Status, transport::Server};

pub mod product {
    tonic::include_proto!("product");
}

use product::product_service_server::{ProductService, ProductServiceServer};
use product::{ProductRequest, ProductResponse};

#[derive(Debug, Default)]
pub struct MyProductService;

#[tonic::async_trait]
impl ProductService for MyProductService {
    async fn get_product(
        &self,
        request: Request<ProductRequest>,
    ) -> Result<Response<ProductResponse>, Status> {
        let id = request.into_inner().id;
        Ok(Response::new(ProductResponse {
            id,
            name: format!("Product #{}", id),
        }))
    }
}

async fn rest_handler() -> String {
    "Hello from REST API".into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let _ = tracing_subscriber::fmt().compact().pretty();

    let grpc_addr: std::net::SocketAddr = env::var("GRPC_ADDR")?.parse()?;
    let rest_addr: std::net::SocketAddr = env::var("REST_ADDR")?.parse()?;

    let rest_app = Router::new()
        .route("/hello", get(rest_handler))
        .route("/", get(|| async { "Hello, World!" }));

    let grpc_service = ProductServiceServer::new(MyProductService::default());
    let grpc = Server::builder().add_service(grpc_service);

    let listener = tokio::net::TcpListener::bind(&rest_addr).await?;

    let (tx, rx) = oneshot::channel::<()>();

    let grpc_handle = tokio::spawn(async move {
        grpc.serve_with_shutdown(grpc_addr, async {
            rx.await.ok();
        })
        .await
        .unwrap();
    });

    let rest_handle = tokio::spawn(async move {
        axum::serve(listener, rest_app).await.unwrap();
    });

    println!("gRPC on {}", grpc_addr);
    println!("REST on {}", rest_addr);

    grpc_handle.await?;
    rest_handle.await?;

    Ok(())
}
