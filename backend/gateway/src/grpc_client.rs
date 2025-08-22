use proto::user::v1::user_service_client::UserServiceClient;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct UserClient(UserServiceClient<Channel>);

impl UserClient {
    pub async fn connect(addr: &str) -> Self {
        let client = UserServiceClient::connect(addr.to_string())
            .await
            .expect("failed to connect user service");
        Self(client)
    }

    pub fn inner(&self) -> UserServiceClient<Channel> {
        self.0.clone()
    }
}
