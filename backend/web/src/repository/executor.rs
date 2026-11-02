use crate::repository::error::Error;
use crate::repository::error::Error::Unexpected;
use sqlx::{PgConnection, Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct Executor {
    pool: Arc<Pool<Postgres>>,
}

impl Executor {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    async fn with_commit<F, T>(self, f: F) -> Result<T, Error>
    where
        F: AsyncFnOnce(&mut PgConnection) -> Result<T, Error>,
    {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Unexpected(e.to_string()))?;

        match f(&mut *tx).await {
            Ok(v) => {
                tx.commit().await.map_err(|e| Unexpected(e.to_string()))?;
                Ok(v)
            }
            Err(e) => {
                tx.rollback().await.map_err(|e| Unexpected(e.to_string()))?;
                Err(e)
            }
        }
    }
}
