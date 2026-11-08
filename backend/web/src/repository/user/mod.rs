use crate::model::Error;
use crate::model::Error::Unexpected;
use crate::model::user::User;
use crate::model::user::repository::UserRepository;
use async_trait::async_trait;
use sqlx::PgConnection;

pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, conn: &mut PgConnection, user: User) -> Result<i64, Error> {
        const SQL: &str = r#"INSERT INTO "user"("name", "email", "password_hash") VALUES($1, $2, $3) RETURNING "user_id""#;

        let user_id = sqlx::query_scalar(SQL)
            .bind(user.name)
            .bind(user.email)
            .bind(user.password_hash)
            .fetch_one(conn)
            .await
            .map_err(|e| Unexpected(e.to_string()))?;

        Ok(user_id)
    }
}
