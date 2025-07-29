use crate::{models::*, repository::*};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_user_service(
    id: uuid::Uuid,
    pool: Arc<PgPool>,
) -> Result<Option<User>, sqlx::Error> {
    get_user_by_id(id, pool).await
}
