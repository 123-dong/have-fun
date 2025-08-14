use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub(crate) struct UserRepo {
    pub pool: Arc<PgPool>,
}

impl UserRepo {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<(i32, String)>, sqlx::Error> {
        sqlx::query_as::<_, (i32, String)>("SELECT id, name FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
    }
}
