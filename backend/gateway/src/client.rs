use proto::user::v1::GetRequest;
use proto::user::v1::user_service_client::UserServiceClient;

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dst = "http://[::1]:50051";

    let mut user_client = UserServiceClient::connect(dst).await?;

    let req = tonic::Request::new(GetRequest { id: 1 });
    let res = user_client.get(req).await?;
    println!("UserService.Get response: {:?}", res.into_inner());

    Ok(())
}
