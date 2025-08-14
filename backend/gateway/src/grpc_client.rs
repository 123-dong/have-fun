use proto::user::v1::user_service_client::UserServiceClient;
use proto::user::v1::{GetRequest, GetResponse};
use tonic::transport::Channel;

pub struct UserGrpcClient {
    inner: UserServiceClient<Channel>,
}

impl UserGrpcClient {
    pub async fn connect(addr: &str) -> Self {
        let client = UserServiceClient::connect(addr.to_string())
            .await
            .expect("Failed to connect to gRPC service");
        Self { inner: client }
    }

    pub async fn get_user(&mut self, id: i32) -> GetResponse {
        let request = tonic::Request::new(GetRequest { id });
        self.inner
            .get(request)
            .await
            .expect("gRPC request failed")
            .into_inner()
    }
}
