use super::repository::UserRepo;
use shared::errors::ServiceError;
use tokio_stream::StreamExt;
use tracing::info;

#[derive(Clone)]
pub(super) struct SvcImpl {
    repo: UserRepo,
}

impl SvcImpl {
    pub(super) fn new(repo: UserRepo) -> Self {
        Self { repo }
    }
}

type StreamResponse = std::pin::Pin<
    Box<dyn tokio_stream::Stream<Item = Result<proto::v1::user::User, tonic::Status>> + Send>,
>;
type StreamResult<T> = Result<tonic::Response<T>, tonic::Status>;

#[tonic::async_trait]
impl proto::v1::user::user_service_server::UserService for SvcImpl {
    async fn create(
        &self,
        request: tonic::Request<proto::v1::user::CreateRequest>,
    ) -> Result<tonic::Response<proto::v1::user::CreateResponse>, tonic::Status> {
        let req = request.into_inner();
        info!(name = %req.name, email = %req.email, "create user request");

        let user: proto::v1::user::User = self
            .repo
            .insert_user(&req.name, &req.email)
            .await
            .map_err(ServiceError::from)?
            .into();

        info!(user_id = %user.id, "user created");
        Ok(tonic::Response::new(proto::v1::user::CreateResponse {
            user: Some(user),
        }))
    }

    async fn get(
        &self,
        request: tonic::Request<proto::v1::user::GetRequest>,
    ) -> Result<tonic::Response<proto::v1::user::GetResponse>, tonic::Status> {
        let id = uuid::Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;
        info!(%id, "get user request");

        let user: proto::v1::user::User = self
            .repo
            .select_user_by_id(id)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?
            .into();

        info!(%id, "user fetched");
        Ok(tonic::Response::new(proto::v1::user::GetResponse {
            user: Some(user),
        }))
    }

    async fn update(
        &self,
        request: tonic::Request<proto::v1::user::UpdateRequest>,
    ) -> Result<tonic::Response<proto::v1::user::UpdateResponse>, tonic::Status> {
        let req = request.into_inner();
        let id = uuid::Uuid::parse_str(&req.id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;
        info!(%id, name = ?req.name, email = ?req.email, "update user request");

        let user: proto::v1::user::User = self
            .repo
            .update_user_by_id(id, req.name, req.email)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?
            .into();

        info!(%id, "user updated");
        Ok(tonic::Response::new(proto::v1::user::UpdateResponse {
            user: Some(user),
        }))
    }

    async fn delete(
        &self,
        request: tonic::Request<proto::v1::user::DeleteRequest>,
    ) -> Result<tonic::Response<proto::v1::user::DeleteResponse>, tonic::Status> {
        let id = uuid::Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;
        info!(%id, "delete user request");

        let deleted = self
            .repo
            .delete_user_by_id(id)
            .await
            .map_err(ServiceError::from)?;

        info!(%id, success = deleted, "user delete result");
        Ok(tonic::Response::new(proto::v1::user::DeleteResponse {
            success: deleted,
        }))
    }

    async fn list(
        &self,
        request: tonic::Request<proto::v1::user::ListRequest>,
    ) -> Result<tonic::Response<proto::v1::user::ListResponse>, tonic::Status> {
        info!(remote = ?request.remote_addr(), "list users request");

        let users = self.repo.list_users().await.map_err(ServiceError::from)?;
        info!(count = users.len(), "users listed");

        Ok(tonic::Response::new(proto::v1::user::ListResponse {
            users: users.into_iter().map(Into::into).collect(),
        }))
    }

    type StreamStream = StreamResponse;
    async fn stream(
        &self,
        request: tonic::Request<proto::v1::user::StreamRequest>,
    ) -> StreamResult<Self::StreamStream> {
        info!(remote = ?request.remote_addr(), "stream users request");

        let stream = self.repo.stream_users().map(|res| {
            res.map(Into::into)
                .map_err(|e| ServiceError::from(e).into())
        });

        Ok(tonic::Response::new(Box::pin(stream) as Self::StreamStream))
    }
}
