use crate::domain::user::User;
use crate::domain::user::repository::UserRepository;

pub(crate) struct UserRepositoryReal;

impl UserRepositoryReal {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl UserRepository for UserRepositoryReal {
    async fn save(&self, user: User) -> Result<i64, sqlx::Error> {
        Ok(1)
    }
}
