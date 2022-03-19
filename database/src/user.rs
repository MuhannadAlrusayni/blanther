use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub phone_number: Option<String>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub username: Option<String>,
}

#[async_trait]
pub trait UserDB {
    async fn insert_user(&self, pool: &Pool<Postgres>) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserDB for User {
    async fn insert_user(&self, pool: &Pool<Postgres>) -> Result<User, sqlx::Error> {
        let rec = sqlx::query_as!(
            User,
            r#"INSERT INTO users (first_name, last_name, email, phone_number, username)
            VALUES ( $1, $2, $3, $4, $5 ) RETURNING *;"#,
            self.first_name,
            self.last_name,
            self.email,
            self.phone_number,
            self.username
        )
        .fetch_one(pool)
        .await?;

        Ok(rec)
    }
}
