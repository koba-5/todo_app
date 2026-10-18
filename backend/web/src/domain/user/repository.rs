use crate::domain::user::User;
use sqlx::PgTransaction;

pub trait UserRepository {
    async fn save(&self, tx: &mut PgTransaction, user: User) -> Result<i64, sqlx::Error>;
}
