use sqlx::{Error, PgPool};

use crate::models::*;

// QUERY constants
const INSERT_USER: &str =
    "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email";
const SELECT_USER_ID: &str = "SELECT id, name, email FROM users WHERE id = $1";
const UPDATE_USER_ID: &str =
    "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email";
const DELETE_USER_ID: &str = "DELETE FROM users WHERE id = $1";

// func
pub async fn init_pool(url: &str, max: u32) -> Result<PgPool, Error> {
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max)
        .connect(url)
        .await?;

    sqlx::migrate!().run(&db).await?;
    Ok(db)
}

pub async fn ping_database(pool: &PgPool) -> Result<(), Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

pub async fn insert_user(pool: &PgPool, new_user: &CreateUser) -> Result<User, Error> {
    sqlx::query_as::<_, User>(INSERT_USER)
        .bind(&new_user.name)
        .bind(&new_user.email)
        .fetch_one(pool)
        .await
}

pub async fn get_user_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<User, Error> {
    sqlx::query_as::<_, User>(SELECT_USER_ID)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update_user_by_id(
    pool: &PgPool,
    id: uuid::Uuid,
    new_user: &CreateUser,
) -> Result<User, Error> {
    sqlx::query_as::<_, User>(UPDATE_USER_ID)
        .bind(id)
        .bind(&new_user.name)
        .bind(&new_user.email)
        .fetch_one(pool)
        .await
}

pub async fn delete_user_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<(), Error> {
    sqlx::query(DELETE_USER_ID).bind(id).execute(pool).await?;
    Ok(())
}
