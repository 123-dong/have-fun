use crate::repository::UserRepo;
use shared::models::DbUser;
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Clone)]
pub struct UserSvc {
    repo: UserRepo,
}

impl UserSvc {
    pub fn new(repo: UserRepo) -> Self {
        Self { repo }
    }

    pub fn list_full(&self) -> Pin<Box<dyn Stream<Item = sqlx::Result<DbUser>> + Send + 'static>> {
        Box::pin(self.repo.list_full())
    }

    pub async fn list_bulk(&self) -> sqlx::Result<Vec<DbUser>> {
        self.repo.list_bulk().await
    }

    pub async fn get_user(&self, id: uuid::Uuid) -> sqlx::Result<Option<DbUser>> {
        self.repo.get(id).await
    }

    pub async fn create_user(&self, name: &str, email: &str) -> sqlx::Result<DbUser> {
        self.repo.create(name, email).await
    }

    pub async fn update_user(
        &self,
        id: uuid::Uuid,
        name: &str,
        email: &str,
    ) -> sqlx::Result<Option<DbUser>> {
        self.repo.update(id, name, email).await
    }

    pub async fn delete_user(&self, id: uuid::Uuid) -> sqlx::Result<bool> {
        self.repo.delete(id).await
    }
}
