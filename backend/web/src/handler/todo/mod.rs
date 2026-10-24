use crate::app::app_state::AppState;
use crate::handler::todo::handler::TodoHandler;
use crate::utils::tx_executor::TxExecutor;
use axum::routing::{MethodRouter, get};
use std::sync::Arc;

mod handler;
mod use_case;

pub fn todo_routes() -> MethodRouter<AppState> {
    get(TodoHandler::get_handler)
}
