use proto::user::v1::{GetRequest, GetResponse, user_service_server::UserService};
use sqlx::PgPool;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub(crate) struct UserSvc {
    pub pool: Arc<PgPool>,
}

#[tonic::async_trait]
impl UserService for UserSvc {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        println!("Got request: {:?}", request);

        let req = request.into_inner();
        let id = req.id;

        let result = sqlx::query_as::<_, (i32, String)>("SELECT id, name FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let user = match result {
            Some((id, name)) => GetResponse { id, name },
            None => return Err(Status::not_found(format!("User with id {} not found", id))),
        };

        Ok(Response::new(user))
    }
}
