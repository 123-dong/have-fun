use tokio_stream::StreamExt;

use super::repository::UserRepo;

#[derive(Clone)]
pub(super) struct SvcImpl {
    repo: UserRepo,
}

impl SvcImpl {
    pub(super) fn new(repo: UserRepo) -> Self {
        Self { repo }
    }
}

#[tonic::async_trait]
impl proto::v1::user::user_service_server::UserService for SvcImpl {
    async fn create(
        &self,
        request: tonic::Request<proto::v1::user::CreateRequest>,
    ) -> Result<tonic::Response<proto::v1::user::CreateResponse>, tonic::Status> {
        let req = request.into_inner();

        let user: proto::v1::user::User = self
            .repo
            .create(&req.name, &req.email)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?
            .into();

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

        let user: proto::v1::user::User = self
            .repo
            .get(id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?
            .ok_or_else(|| tonic::Status::not_found("user not found"))?
            .into();

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

        let user: proto::v1::user::User = self
            .repo
            .update(id, req.name, req.email)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?
            .ok_or_else(|| tonic::Status::not_found("user not found"))?
            .into();

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

        let deleted = self
            .repo
            .delete(id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(proto::v1::user::DeleteResponse {
            success: deleted,
        }))
    }

    async fn list_bulk(
        &self,
        _request: tonic::Request<proto::v1::user::ListBulkRequest>,
    ) -> Result<tonic::Response<proto::v1::user::ListBulkResponse>, tonic::Status> {
        let users = self
            .repo
            .list_bulk()
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(proto::v1::user::ListBulkResponse {
            users: users.into_iter().map(Into::into).collect(),
        }))
    }

    type ListFullStream = std::pin::Pin<
        Box<dyn tokio_stream::Stream<Item = Result<proto::v1::user::User, tonic::Status>> + Send>,
    >;

    async fn list_full(
        &self,
        _request: tonic::Request<proto::v1::user::ListFullRequest>,
    ) -> Result<tonic::Response<Self::ListFullStream>, tonic::Status> {
        let stream = self.repo.list_full().map(|res| match res {
            Ok(u) => Ok(u.into()),
            Err(e) => Err(tonic::Status::internal(e.to_string())),
        });

        Ok(tonic::Response::new(
            Box::pin(stream) as Self::ListFullStream
        ))
    }
}
