#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
}

impl From<UserModel> for proto::user::v1::User {
    fn from(u: UserModel) -> Self {
        Self {
            id: u.id.to_string(),
            name: u.name,
            email: u.email,
        }
    }
}
