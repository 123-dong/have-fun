use crate::models::{NewUser, User};
use sqlx::PgPool;
use std::sync::Arc;

const SELECT_USER_BY_ID: &str = r#"
    SELECT id, name, email
    FROM users
    WHERE id = $1
"#;

pub async fn get_user_by_id(
    id: uuid::Uuid,
    pool: Arc<PgPool>,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&*pool)
        .await
}
