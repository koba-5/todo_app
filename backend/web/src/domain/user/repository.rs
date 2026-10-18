use crate::domain::user::User;

pub trait UserRepository {
    async fn save(&self, user: User) -> Result<i64, sqlx::Error>;
}
