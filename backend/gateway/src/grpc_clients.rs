use proto::v1::user::user_service_client::UserServiceClient;
use std::sync::Arc;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct GrpcClients {
    pub user: UserServiceClient<Channel>,
    // pub order: OrderServiceClient<Channel>,
}

impl GrpcClients {
    pub async fn new(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(addr)?.connect().await?;

        Ok(Self {
            user: UserServiceClient::new(channel.clone()),
            // order: OrderServiceClient::new(channel.clone()),
        })
    }
}

#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<GrpcClients>,
}

impl AppState {
    pub async fn new(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let clients = GrpcClients::new(addr).await?;
        Ok(Self {
            clients: Arc::new(clients),
        })
    }
}
