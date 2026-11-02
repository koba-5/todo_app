use crate::repository::executor::Executor;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    executor: Arc<Executor>,
}

impl AppState {
    pub fn new(executor: Arc<Executor>) -> Self {
        Self { executor }
    }
}
