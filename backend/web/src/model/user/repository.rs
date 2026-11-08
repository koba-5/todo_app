use crate::model::Error;
use crate::model::user::User;
use async_trait::async_trait;
use sqlx::PgConnection;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, user: User) -> Result<i64, Error>;
}
