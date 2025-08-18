use crate::repository::UserRepo;
use shared::models::UserModel;

pub(crate) struct UserSvc {
    repo: UserRepo,
}

impl UserSvc {
    pub fn new(repo: UserRepo) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, name: &str, email: &str) -> sqlx::Result<UserModel> {
        self.repo.create(name, email).await
    }
    pub async fn list_users(&self) -> sqlx::Result<Vec<UserModel>> {
        self.repo.list().await
    }
    pub async fn get_user(&self, id: uuid::Uuid) -> sqlx::Result<Option<UserModel>> {
        self.repo.get(id).await
    }
    pub async fn delete_user(&self, id: uuid::Uuid) -> sqlx::Result<bool> {
        self.repo.delete(id).await
    }
    pub async fn update_user(
        &self,
        id: uuid::Uuid,
        name: &str,
        email: &str,
    ) -> sqlx::Result<Option<UserModel>> {
        self.repo.update(id, name, email).await
    }
}
