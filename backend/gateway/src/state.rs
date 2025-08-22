use crate::grpc_client::UserClient;

#[derive(Clone)]
pub struct AppState {
    pub user: UserClient,
}

impl AppState {
    pub async fn init() -> Self {
        Self {
            user: UserClient::connect("http://[::1]:50051").await,
        }
    }
}
