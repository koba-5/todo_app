use crate::handler::user::handler::UserHandler;
use axum::routing::{MethodRouter, get};

mod handler;
mod use_case;

pub fn user_routes() -> MethodRouter {
    get(UserHandler::get_handler)
}
