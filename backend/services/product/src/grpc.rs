use crate::handler::AppState;
use crate::health::health_server::{Health, HealthServer};
use crate::health::{HealthReply, HealthRequest};
use tonic::{Request, Response, Status};
use tracing::info;

pub struct MyHealth {
    pub state: AppState,
}

#[tonic::async_trait]
impl Health for MyHealth {
    async fn health_check(
        &self,
        request: Request<HealthRequest>,
    ) -> Result<Response<HealthReply>, Status> {
        let name = request.into_inner().name;
        let msg = self
            .state
            .do_health_check(&name)
            .await
            .map_err(|_| Status::internal("DB error"))?;
        info!("Handled gRPC health_check for {}", name);
        Ok(Response::new(HealthReply { message: msg }))
    }
}

pub fn create_grpc_server(state: AppState) -> HealthServer<MyHealth> {
    HealthServer::new(MyHealth { state })
}
