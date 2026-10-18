use crate::utils::tx_manager::config::Config;
use sqlx::{PgPool, PgTransaction};

mod config;

pub(crate) struct TxManager {
    pool: PgPool,
}

impl TxManager {
    pub async fn new() -> Self {
        let config = Config::new();
        Self {
            pool: PgPool::connect(&config.database_url).await.unwrap(),
        }
    }

    pub async fn execute<F, T>(&self, f: F) -> Result<T, sqlx::Error>
    where
        F: AsyncFnOnce(&mut PgTransaction) -> Result<T, sqlx::Error>,
    {
        let mut tx = self.pool.begin().await?;
        match f(&mut tx).await {
            Ok(v) => {
                tx.commit().await?;
                Ok(v)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}
