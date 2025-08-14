use crate::repository::UserRepo;
use proto::user::v1::{GetRequest, GetResponse, user_service_server::UserService};
use tonic::{Request, Response, Status};
use tracing::info;

#[derive(Debug, Clone)]
pub(crate) struct UserSvc {
    pub repo: UserRepo,
}

#[tonic::async_trait]
impl UserService for UserSvc {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        info!("Got request: {:?}", request);

        let req = request.into_inner();
        let id = req.id;

        let result = self
            .repo
            .get_user(id)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let user = match result {
            Some((id, name)) => GetResponse { id, name },
            None => return Err(Status::not_found(format!("User with id {} not found", id))),
        };

        Ok(Response::new(user))
    }
}
