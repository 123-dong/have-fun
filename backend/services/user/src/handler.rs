use crate::service::UserSvc;
use proto::user::v1::user_service_server::UserService;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct UserHdl {
    svc: UserSvc,
}

impl UserHdl {
    pub fn new(svc: UserSvc) -> Self {
        Self { svc }
    }
}

#[tonic::async_trait]
impl UserService for UserHdl {
    type ListFullStream = std::pin::Pin<
        Box<
            dyn tokio_stream::Stream<Item = Result<proto::user::v1::User, tonic::Status>>
                + Send
                + 'static,
        >,
    >;

    async fn list_full(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::ListFullStream>, tonic::Status> {
        let stream = self.svc.list_full().map(|res| {
            res.map(proto::user::v1::User::from)
                .map_err(|e| tonic::Status::internal(e.to_string()))
        });

        Ok(tonic::Response::new(Box::pin(stream)))
    }

    async fn list_bulk(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<proto::user::v1::ListBulkResponse>, tonic::Status> {
        let users = self
            .svc
            .list_bulk()
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(proto::user::v1::ListBulkResponse {
            users: users.into_iter().map(Into::into).collect(),
        }))
    }

    async fn get(
        &self,
        request: tonic::Request<proto::user::v1::GetRequest>,
    ) -> Result<tonic::Response<proto::user::v1::User>, tonic::Status> {
        let id = uuid::Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;

        let user = self
            .svc
            .get_user(id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?
            .ok_or_else(|| tonic::Status::not_found("user not found"))?;

        Ok(tonic::Response::new(user.into()))
    }

    async fn create(
        &self,
        request: tonic::Request<proto::user::v1::CreateRequest>,
    ) -> Result<tonic::Response<proto::user::v1::User>, tonic::Status> {
        let req = request.into_inner();

        let user = self
            .svc
            .create_user(&req.name, &req.email)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(user.into()))
    }

    async fn update(
        &self,
        request: tonic::Request<proto::user::v1::UpdateRequest>,
    ) -> Result<tonic::Response<proto::user::v1::User>, tonic::Status> {
        let req = request.into_inner();
        let id = uuid::Uuid::parse_str(&req.id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;

        let user = self
            .svc
            .update_user(id, &req.name, &req.email)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?
            .ok_or_else(|| tonic::Status::not_found("user not found"))?;

        Ok(tonic::Response::new(user.into()))
    }

    async fn delete(
        &self,
        request: tonic::Request<proto::user::v1::GetRequest>,
    ) -> Result<tonic::Response<proto::user::v1::DeleteResponse>, tonic::Status> {
        let id = uuid::Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;

        let deleted = self
            .svc
            .delete_user(id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(proto::user::v1::DeleteResponse {
            success: deleted,
        }))
    }
}
