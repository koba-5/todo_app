use crate::handler::user::handler::UserHandler;
use crate::utils::tx_executor::TxExecutor;
use axum::routing::{MethodRouter, get};
use std::sync::Arc;

mod get_response;
mod handler;
mod post_request;
mod use_case;

pub fn user_routes() -> MethodRouter<Arc<TxExecutor>> {
    get(UserHandler::get_handler).post(UserHandler::post_handler)
}
