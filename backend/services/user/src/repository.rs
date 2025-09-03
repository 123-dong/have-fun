use async_stream::try_stream;
use shared::database::DbPool; // type DbPool = Arc<PgPool>
use shared::models::DbUser;
use tokio_stream::StreamExt;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepo {
    pool: DbPool,
}

impl UserRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn list_full(
        &self,
    ) -> impl tokio_stream::Stream<Item = sqlx::Result<DbUser>> + Send + 'static {
        let pool = self.pool.clone();
        try_stream! {
            let mut rows = sqlx::query_as!(
                DbUser,
                r#"SELECT id, name, email FROM users ORDER BY name"#
            )
            .fetch(&*pool);

            while let Some(row) = rows.next().await {
                yield row?;
            }
        }
    }

    // pub fn list_full(&self) -> impl tokio_stream::Stream<Item = sqlx::Result<DbUser>> + '_ {
    //     sqlx::query_as::<_, DbUser>("SELECT id, name, email FROM users").fetch(&*self.pool)
    // }

    pub async fn list_bulk(&self) -> sqlx::Result<Vec<DbUser>> {
        sqlx::query_as!(
            DbUser,
            r#"
            SELECT id, name, email
            FROM users
            ORDER BY name
            "#
        )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn get(&self, id: Uuid) -> sqlx::Result<Option<DbUser>> {
        sqlx::query_as!(
            DbUser,
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

    pub async fn create(&self, name: &str, email: &str) -> sqlx::Result<DbUser> {
        sqlx::query_as!(
            DbUser,
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

    pub async fn update(&self, id: Uuid, name: &str, email: &str) -> sqlx::Result<Option<DbUser>> {
        sqlx::query_as!(
            DbUser,
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

    pub async fn delete(&self, id: Uuid) -> sqlx::Result<bool> {
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
}
