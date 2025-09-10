use async_stream::try_stream;
use tokio_stream::StreamExt;
use uuid::Uuid;

use shared::database::DbPool;
use shared::errors::AppError;
use shared::models::DbUser;
use tracing::{error, info};

#[derive(Clone)]
pub(super) struct UserRepo {
    pool: DbPool,
}

impl UserRepo {
    pub(super) fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub(super) fn stream_all_users(
        &self,
    ) -> impl tokio_stream::Stream<Item = Result<DbUser, AppError>> + Send + 'static {
        let pool = self.pool.clone();
        try_stream! {
            let mut rows = sqlx::query_as!(
                DbUser,
                "SELECT id, name, email FROM users ORDER BY name"
            )
            .fetch(&*pool);

            while let Some(row) = rows.next().await {
                let u = row?;
                info!(user_id = %u.id, user_name = %u.name, "Streaming user");
                yield u;
            }
        }
    }

    pub(super) async fn list_all_users(&self) -> Result<Vec<DbUser>, AppError> {
        info!("Listing all users");
        sqlx::query_as!(DbUser, "SELECT id, name, email FROM users ORDER BY name")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| {
                error!("Failed to list all users: {:?}", e);
                AppError::from(e)
            })
    }

    pub(super) async fn select_user_by_id(&self, id: Uuid) -> Result<Option<DbUser>, AppError> {
        info!(user_id = %id, "Selecting user by ID");
        sqlx::query_as!(
            DbUser,
            "SELECT id, name, email FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| {
            error!(user_id = %id, "Failed to select user: {:?}", e);
            AppError::from(e)
        })
    }

    pub(super) async fn insert_user(&self, name: &str, email: &str) -> Result<DbUser, AppError> {
        info!(user_name = %name, user_email = %email, "Inserting new user");
        sqlx::query_as!(
            DbUser,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
            name,
            email
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            error!(user_name = %name, user_email = %email, "Failed to insert user: {:?}", e);
            AppError::from(e)
        })
    }

    // TODO: fix sticky name, email field
    pub(super) async fn update_user_by_id(
        &self,
        id: Uuid,
        name: Option<String>,
        email: Option<String>,
    ) -> Result<Option<DbUser>, AppError> {
        info!(user_id = %id, "Updating user");
        sqlx::query_as!(
            DbUser,
            "UPDATE users SET name = $2, email = $3 WHERE id = $1 RETURNING id, name, email",
            id,
            name,
            email
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| {
            error!(user_id = %id, "Failed to update user: {:?}", e);
            AppError::from(e)
        })
    }

    pub(super) async fn delete_user_by_id(&self, id: Uuid) -> Result<bool, AppError> {
        info!(user_id = %id, "Deleting user");
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!(user_id = %id, "Failed to delete user: {:?}", e);
                AppError::from(e)
            })?;

        Ok(result.rows_affected() > 0)
    }
}
