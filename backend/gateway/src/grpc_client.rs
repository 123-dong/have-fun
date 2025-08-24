use std::sync::Arc;
use tonic::transport::Channel;

use proto::user::v1::user_service_client::UserServiceClient;

#[derive(Clone)]
pub struct Clients {
    pub user: UserServiceClient<Channel>,
}

impl Clients {
    pub async fn init(user_addr: &str) -> tonic::Result<Self> {
        let user = UserServiceClient::connect(user_addr.to_string())
            .await
            .unwrap();

        Ok(Self { user })
    }

    pub fn shared(self) -> Arc<Self> {
        Arc::new(self)
    }
}
