use crate::app::jwt::Jwt;
use crate::model::event_log::repository::EventLogRepository;
use crate::repository::executor::Executor;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub jwt: Arc<Jwt>,
    pub(crate) executor: Arc<Executor>,
    pub(crate) event_log_repository: Arc<dyn EventLogRepository>,
}

impl AppState {
    pub fn new(
        jwt: Arc<Jwt>,
        executor: Arc<Executor>,
        event_log_repository: Arc<dyn EventLogRepository>,
    ) -> Self {
        Self {
            jwt,
            executor,
            event_log_repository,
        }
    }
}
