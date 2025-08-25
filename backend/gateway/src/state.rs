use crate::grpc_clients::GrpcClients;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<GrpcClients>,
}
