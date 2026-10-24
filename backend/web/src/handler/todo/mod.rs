use crate::app::app_state::AppState;
use crate::handler::todo::handler::TodoHandler;
use axum::routing::{MethodRouter, get};

mod handler;
mod use_case;

pub fn todo_routes() -> MethodRouter<AppState> {
    get(TodoHandler::get_handler)
}
