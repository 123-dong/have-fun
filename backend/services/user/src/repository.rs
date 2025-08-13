// use sqlx::{Pool, Postgres};
// use tonic::{Request, Response, Status};
// use uuid::Uuid;

// use crate::proto::{
//     CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, GetRequest, GetResponse,
//     ListRequest, ListResponse, UpdateRequest, UpdateResponse, User,
//     user_service_server::UserService,
// };

// struct UserRepository {
//     pool: Pool<Postgres>,
// }

// impl UserRepository {
//     fn new(pool: Pool<Postgres>) -> Self {
//         Self { pool }
//     }

//     async fn create(&self, name: &str) -> Result<User, ServiceError> {
//         let id = Uuid::new_v4();
//         sqlx::query!(
//             "INSERT INTO users (id, name) VALUES ($1, $2) RETURNING id, name",
//             id,
//             name
//         )
//         .fetch_one(&self.pool)
//         .await
//         .map(|row| User {
//             id: row.id.to_string(),
//             name: row.name,
//         })
//         .map_err(ServiceError::Database)
//     }

//     async fn get(&self, id: &str) -> Result<User, ServiceError> {
//         let uuid = Uuid::parse_str(id).map_err(|_| ServiceError::InvalidUuid)?;
//         sqlx::query!("SELECT id, name FROM users WHERE id = $1", uuid)
//             .fetch_one(&self.pool)
//             .await
//             .map(|row| User {
//                 id: row.id.to_string(),
//                 name: row.name,
//             })
//             .map_err(|_| ServiceError::NotFound)
//     }

//     async fn update(&self, id: &str, name: &str) -> Result<User, ServiceError> {
//         let uuid = Uuid::parse_str(id).map_err(|_| ServiceError::InvalidUuid)?;
//         sqlx::query!(
//             "UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name",
//             name,
//             uuid
//         )
//         .fetch_one(&self.pool)
//         .await
//         .map(|row| User {
//             id: row.id.to_string(),
//             name: row.name,
//         })
//         .map_err(|_| ServiceError::NotFound)
//     }

//     async fn delete(&self, id: &str) -> Result<bool, ServiceError> {
//         let uuid = Uuid::parse_str(id).map_err(|_| ServiceError::InvalidUuid)?;
//         let result = sqlx::query!("DELETE FROM users WHERE id = $1", uuid)
//             .execute(&self.pool)
//             .await
//             .map_err(ServiceError::Database)?;
//         Ok(result.rows_affected() > 0)
//     }

//     async fn list(&self) -> Result<Vec<User>, ServiceError> {
//         sqlx::query!("SELECT id, name FROM users")
//             .fetch_all(&self.pool)
//             .await
//             .map(|rows| {
//                 rows.into_iter()
//                     .map(|r| User {
//                         id: r.id.to_string(),
//                         name: r.name,
//                     })
//                     .collect()
//             })
//             .map_err(ServiceError::Database)
//     }
// }
