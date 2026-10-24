use crate::app::app_state::AppState;
use crate::handler::user::handler::UserHandler;
use axum::routing::{MethodRouter, get};

mod get_response;
mod handler;
mod post_request;
mod post_response;
mod use_case;

pub fn user_routes() -> MethodRouter<AppState> {
    get(UserHandler::get_handler).post(UserHandler::post_handler)
}
