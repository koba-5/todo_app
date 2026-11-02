use crate::app::app_state::AppState;
use crate::handler::event_log::post_handler;
use axum::Router;
use axum::routing::post;

pub mod app_state;

pub async fn build(app_state: AppState) -> Router {
    Router::new()
        .route("/event_log", post(post_handler))
        .with_state(app_state)
}
