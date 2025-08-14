use serde::Serialize;

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: i32,
    pub name: String,
}

impl From<proto::user::v1::GetResponse> for GetUserResponse {
    fn from(resp: proto::user::v1::GetResponse) -> Self {
        Self {
            id: resp.id,
            name: resp.name,
        }
    }
}
