use crate::service::UserSvc;
use futures::Stream;
use proto::user::v1::user_service_server::UserService;

pub(crate) struct UserHdl {
    service: UserSvc,
}

impl UserHdl {
    pub fn new(service: UserSvc) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl UserService for UserHdl {
    type ListStream = std::pin::Pin<
        Box<dyn Stream<Item = Result<proto::user::v1::User, tonic::Status>> + Send + 'static>,
    >;

    async fn create(
        &self,
        request: tonic::Request<proto::user::v1::CreateRequest>,
    ) -> Result<tonic::Response<proto::user::v1::User>, tonic::Status> {
        let req = request.into_inner();

        let user = self
            .service
            .create_user(&req.name, &req.email)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(user.into()))
    }

    async fn get(
        &self,
        request: tonic::Request<proto::user::v1::GetRequest>,
    ) -> Result<tonic::Response<proto::user::v1::User>, tonic::Status> {
        let id = uuid::Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;

        let user = self
            .service
            .get_user(id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?
            .ok_or_else(|| tonic::Status::not_found("user not found"))?;

        Ok(tonic::Response::new(user.into()))
    }

    async fn list(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::ListStream>, tonic::Status> {
        let users = self
            .service
            .list_users()
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        // convert Vec<UserModel> -> stream of proto::User
        let output = futures::stream::iter(users.into_iter().map(|u| Ok(u.into())));
        Ok(tonic::Response::new(Box::pin(output)))
    }

    async fn delete(
        &self,
        request: tonic::Request<proto::user::v1::GetRequest>,
    ) -> Result<tonic::Response<proto::user::v1::DeleteResponse>, tonic::Status> {
        let id = uuid::Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;

        let deleted = self
            .service
            .delete_user(id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(proto::user::v1::DeleteResponse {
            success: deleted,
        }))
    }

    async fn update(
        &self,
        request: tonic::Request<proto::user::v1::UpdateRequest>,
    ) -> Result<tonic::Response<proto::user::v1::User>, tonic::Status> {
        let req = request.into_inner();
        let id = uuid::Uuid::parse_str(&req.id)
            .map_err(|_| tonic::Status::invalid_argument("invalid uuid"))?;

        let user = self
            .service
            .update_user(id, &req.name, &req.email)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?
            .ok_or_else(|| tonic::Status::not_found("user not found"))?;

        Ok(tonic::Response::new(user.into()))
    }
}
