use crate::model::Error;
use crate::model::Error::Unexpected;
use crate::model::event_log::EventLog;
use crate::model::event_log::repository::EventLogRepository;
use async_trait::async_trait;
use sqlx::PgConnection;

pub struct EventLogRepositoryImpl;

#[async_trait]
impl EventLogRepository for EventLogRepositoryImpl {
    async fn create(&self, conn: &mut PgConnection, event_log: EventLog) -> Result<(), Error> {
        const SQL: &str = r#"INSERT INTO "event_log"("session_id", "user_id", "event_type", "event_name") VALUES($1, $2, $3, $4)"#;

        sqlx::query(SQL)
            .bind(event_log.session_id)
            .bind(event_log.user_id)
            .bind(event_log.event_type)
            .bind(event_log.event_name)
            .execute(conn)
            .await
            .map_err(|_| Unexpected("".to_string()))?;

        Ok(())
    }
}
