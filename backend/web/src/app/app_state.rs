use crate::db::Executor;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub(crate) executor: Arc<Executor>,
}

impl AppState {
    pub fn new(executor: Arc<Executor>) -> Self {
        Self { executor }
    }
}
