use proto::user::v1::user_service_client::UserServiceClient;
use std::sync::Arc;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct GrpcClients {
    pub user: UserServiceClient<Channel>,
}

#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<GrpcClients>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let user_client = UserServiceClient::connect("http://127.0.0.1:50051").await?;
        let clients = GrpcClients { user: user_client };

        Ok(Self {
            clients: Arc::new(clients),
        })
    }
}
