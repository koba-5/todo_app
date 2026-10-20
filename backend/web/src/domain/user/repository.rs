use crate::domain::user::User;
use sqlx::PgTransaction;

pub(crate) trait UserRepository {
    async fn get_by_id(&self, tx: &mut PgTransaction, user_id: i64) -> Result<User, sqlx::Error>;
    async fn save(&self, tx: &mut PgTransaction, user: User) -> Result<i64, sqlx::Error>;
}
