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
    async fn get_by_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_,User>(r#"SELECT "user_id", "name", "email", "password" FROM public."user" WHERE "user_id" = $1"#)
            .bind(user_id)
            .fetch_one(&mut **tx)
            .await?;

        Ok(user)
    }

    async fn save(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: User,
    ) -> Result<i64, sqlx::Error> {
        let user_id = sqlx::query_scalar::<_, i64>(r#"INSERT INTO public."user" ("name", "email", "password") VALUES ($1, $2, $3) RETURNING user_id"#)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password)
            .fetch_one(&mut **tx)
            .await?;
        Ok(user_id)
    }
}
