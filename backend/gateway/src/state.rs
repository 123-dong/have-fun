use crate::grpc_client::GrpcClients;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<GrpcClients>,
}
