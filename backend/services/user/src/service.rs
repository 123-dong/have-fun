use crate::repository::UserRepo;
use shared::models::UserModel;

#[derive(Clone)]
pub struct UserSvc {
    repo: UserRepo,
}

impl UserSvc {
    pub fn new(repo: UserRepo) -> Self {
        Self { repo }
    }

    pub async fn list_bulk(&self) -> sqlx::Result<Vec<UserModel>> {
        self.repo.list_bulk().await
    }

    pub fn list_full(
        &self,
    ) -> impl tokio_stream::Stream<Item = sqlx::Result<UserModel>> + Send + 'static {
        self.repo.list_full()
    }

    pub async fn get_user(&self, id: uuid::Uuid) -> sqlx::Result<Option<UserModel>> {
        self.repo.get(id).await
    }

    pub async fn create_user(&self, name: &str, email: &str) -> sqlx::Result<UserModel> {
        self.repo.create(name, email).await
    }

    pub async fn update_user(
        &self,
        id: uuid::Uuid,
        name: &str,
        email: &str,
    ) -> sqlx::Result<Option<UserModel>> {
        self.repo.update(id, name, email).await
    }

    pub async fn delete_user(&self, id: uuid::Uuid) -> sqlx::Result<bool> {
        self.repo.delete(id).await
    }
}
