use crate::model::event_log::repository::EventLogRepository;
use crate::repository::executor::Executor;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub(crate) executor: Arc<Executor>,
    pub(crate) event_log_repository: Arc<dyn EventLogRepository>,
}

impl AppState {
    pub fn new(executor: Arc<Executor>, event_log_repository: Arc<dyn EventLogRepository>) -> Self {
        Self {
            executor,
            event_log_repository,
        }
    }
}
