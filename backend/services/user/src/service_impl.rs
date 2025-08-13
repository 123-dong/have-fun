use proto::user::v1::{GetRequest, GetResponse, user_service_server::UserService};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

#[allow(unused_imports)]
use uuid::Uuid;

#[derive(Debug)]
pub struct UserSvc {
    pub pool: PgPool,
}

#[tonic::async_trait]
impl UserService for UserSvc {
    async fn get(&self, _request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        sqlx::query!("SELECT 1 as one")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Status::internal(format!("DB connection error: {}", e)))?;

        Ok(Response::new(GetResponse {
            id: 1,
            name: "DB connection OK".to_string(),
        }))
    }
}
