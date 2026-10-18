use crate::domain::user::User;
use crate::domain::user::repository::UserRepository;
use sqlx::{Postgres, Transaction};

pub(crate) struct UserRepositoryReal;

impl UserRepositoryReal {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl UserRepository for UserRepositoryReal {
    async fn save(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: User,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query(r#"INSERT INTO public."user" ("name", "email", "password") VALUES ($1, $2, $3) RETURNING user_id"#)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password)
            .fetch_one(&mut **tx)
            .await?;
        Ok(1)
    }
}
