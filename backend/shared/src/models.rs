#[derive(Debug, sqlx::FromRow)]
pub struct DbUser {
    pub id: uuid::Uuid,
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
