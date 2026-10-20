use crate::handler::user::handler::UserHandler;
use axum::routing::{MethodRouter, get};

mod get_response;
mod handler;
mod post_request;
mod use_case;

pub fn user_routes() -> MethodRouter {
    get(UserHandler::get_handler).post(UserHandler::post_handler)
}
