use crate::handler::todo::handler::TodoHandler;
use axum::routing::{MethodRouter, get};

mod handler;
mod use_case;

pub fn todo_routes() -> MethodRouter {
    get(TodoHandler::get_handler)
}
