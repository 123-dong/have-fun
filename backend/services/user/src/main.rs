use proto::DESCRIPTOR_SET;
use proto::user::v1::{
    GetResponse,
    user_service_server::{UserService, UserServiceServer},
};

// tonic
use tonic::{Response, transport::Server};
use tonic_reflection::server::Builder;

#[derive(Debug, Default)]
pub struct User;

#[tonic::async_trait]
impl UserService for User {
    async fn get(
        &self,
        request: tonic::Request<proto::user::v1::GetRequest>,
    ) -> std::result::Result<tonic::Response<proto::user::v1::GetResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);
        let reply = GetResponse {
            id: request.into_inner().id,
            name: "Health".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = User::default();

    let reflection = Builder::configure()
        .register_encoded_file_descriptor_set(DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("UserService listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(service))
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}
