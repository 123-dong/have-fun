use proto::user::v1::user_service_client::UserServiceClient;
use tonic::transport::Channel;

pub async fn new() -> UserServiceClient<Channel> {
    UserServiceClient::connect("http://[::1]:50051")
        .await
        .expect("Failed to connect to User Service")
}
