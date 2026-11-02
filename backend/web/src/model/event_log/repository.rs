use crate::model::Error;
use crate::model::event_log::EventLog;
use async_trait::async_trait;
use sqlx::PgConnection;

#[async_trait]
pub trait EventLogRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, event_log: EventLog) -> Result<(), Error>;
}
