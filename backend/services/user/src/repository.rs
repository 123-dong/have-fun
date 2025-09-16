use shared::database::DbPool; // alias for PgPool
use shared::errors::AppError;
use tokio_stream::StreamExt;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Clone, Debug, sqlx::FromRow, serde::Serialize)]
pub struct DbUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<DbUser> for proto::v1::user::User {
    fn from(u: DbUser) -> Self {
        Self {
            id: u.id.to_string(),
            name: u.name,
            email: u.email,
        }
    }
}

#[derive(Clone)]
pub(super) struct UserRepo {
    pool: DbPool,
}

impl UserRepo {
    pub(super) fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // --- SQL Constants ---
    const SQL_SELECT_ALL: &'static str = "SELECT id, name, email FROM users ORDER BY name";
    const SQL_SELECT_BY_ID: &'static str = "SELECT id, name, email FROM users WHERE id = $1";
    const SQL_INSERT: &'static str =
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email";
    const SQL_UPDATE_BY_ID: &'static str =
        "UPDATE users SET name = $2, email = $3 WHERE id = $1 RETURNING id, name, email";
    const SQL_DELETE_BY_ID: &'static str = "DELETE FROM users WHERE id = $1";

    // --- Common Error Mapper ---
    fn map_db_err(context: &'static str) -> impl FnOnce(sqlx::Error) -> AppError {
        move |e| {
            error!("{context}: {:?}", e);
            AppError::from(e)
        }
    }

    // --- Stream Users ---
    pub fn stream_users(
        &self,
    ) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = Result<DbUser, AppError>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async_stream::try_stream! {
            let mut rows = sqlx::query_as!(DbUser, "SELECT id, name, email FROM users ORDER BY name")
                .fetch(&pool);

            while let Some(user) = rows.next().await {
                yield user.map_err(AppError::from)?;
            }
        })
    }

    // --- List Users ---
    pub(super) async fn list_users(&self) -> Result<Vec<DbUser>, AppError> {
        info!("Listing all users");
        sqlx::query_as::<_, DbUser>(Self::SQL_SELECT_ALL)
            .fetch_all(&self.pool)
            .await
            .map_err(Self::map_db_err("List users failed"))
    }

    // --- Select User by ID ---
    pub(super) async fn select_user_by_id(&self, id: Uuid) -> Result<Option<DbUser>, AppError> {
        info!(user_id = %id, "Selecting user by ID");
        sqlx::query_as::<_, DbUser>(Self::SQL_SELECT_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Self::map_db_err("Select user by ID failed"))
    }

    // --- Insert User ---
    pub(super) async fn insert_user(&self, name: &str, email: &str) -> Result<DbUser, AppError> {
        info!(user_name = %name, user_email = %email, "Inserting new user");

        sqlx::query_as::<_, DbUser>(Self::SQL_INSERT)
            .bind(name)
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if let Some(db_err) = e.as_database_error() {
                    // email UNIQUE handler
                    if db_err.code().map(|c| c == "23505").unwrap_or(false) {
                        error!("Duplicate email: {}", email);
                        return AppError::Conflict("Email already exists".to_string());
                    }
                }
                error!("Insert user failed: {:?}", e);
                AppError::from(e)
            })
    }

    // --- Update User ---
    pub(super) async fn update_user_by_id(
        &self,
        id: Uuid,
        name: String,
        email: String,
    ) -> Result<Option<DbUser>, AppError> {
        info!(user_id = %id, "Updating user");
        sqlx::query_as::<_, DbUser>(Self::SQL_UPDATE_BY_ID)
            .bind(id)
            .bind(name)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(Self::map_db_err("Update user failed"))
    }

    // --- Delete User ---
    pub(super) async fn delete_user_by_id(&self, id: Uuid) -> Result<bool, AppError> {
        info!(user_id = %id, "Deleting user");
        let result = sqlx::query(Self::SQL_DELETE_BY_ID)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Self::map_db_err("Delete user failed"))?;

        Ok(result.rows_affected() > 0)
    }
}
