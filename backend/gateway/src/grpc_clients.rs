use std::sync::Arc;
use tonic::transport::Channel;
use tracing::info;

fn connect_lazy(addr: impl Into<String>, service_name: &str) -> Channel {
    let addr = addr.into();
    let endpoint =
        tonic::transport::Endpoint::from_shared(addr.clone()).expect("Invalid gRPC endpoint");

    info!("Lazily connecting to {} at {}", service_name, addr);
    endpoint.connect_lazy()
}

#[derive(Clone)]
pub struct GrpcClients {
    pub user: proto::v1::user::user_service_client::UserServiceClient<Channel>,
}

impl GrpcClients {
    pub fn new(user_addr: impl Into<String>) -> Self {
        Self {
            user: proto::v1::user::user_service_client::UserServiceClient::new(connect_lazy(
                user_addr,
                "UserService",
            )),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<GrpcClients>,
}

impl AppState {
    pub fn new(user_addr: impl Into<String>) -> Self {
        Self {
            clients: Arc::new(GrpcClients::new(user_addr)),
        }
    }
}
