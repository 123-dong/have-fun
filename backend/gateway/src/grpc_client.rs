use std::sync::Arc;
use tonic::transport::Channel;

use proto::user::v1::user_service_client::UserServiceClient;

#[derive(Clone)]
pub struct GrpcClients {
    pub user: UserServiceClient<Channel>,
    // pub order: OrderClient<Channel>,
}

impl GrpcClients {
    pub async fn new() -> Arc<Self> {
        let user = UserServiceClient::connect("http://[::1]:50051")
            .await
            .expect("connect user service");

        // let order = OrderClient::connect("http://order-service:50052")
        //     .await
        //     .expect("connect order service");

        Arc::new(Self { user })
    }
}
