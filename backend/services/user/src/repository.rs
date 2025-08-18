use shared::database::DbPool;
use shared::models::UserModel;

pub(crate) struct UserRepo {
    pool: DbPool,
}

impl UserRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, name: &str, email: &str) -> sqlx::Result<UserModel> {
        sqlx::query_as!(
            UserModel,
            r#"
            INSERT INTO users (name, email)
            VALUES ($1, $2)
            RETURNING id, name, email
            "#,
            name,
            email
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn get(&self, id: uuid::Uuid) -> sqlx::Result<Option<UserModel>> {
        sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, name, email
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn update(
        &self,
        id: uuid::Uuid,
        name: &str,
        email: &str,
    ) -> sqlx::Result<Option<UserModel>> {
        sqlx::query_as!(
            UserModel,
            r#"
            UPDATE users
            SET name = $2, email = $3
            WHERE id = $1
            RETURNING id, name, email
            "#,
            id,
            name,
            email
        )
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn delete(&self, id: uuid::Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn list(&self) -> sqlx::Result<Vec<UserModel>> {
        sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, name, email
            FROM users
            ORDER BY name
            "#
        )
        .fetch_all(&*self.pool)
        .await
    }
}
