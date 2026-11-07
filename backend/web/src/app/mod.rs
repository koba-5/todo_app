use crate::app::app_state::AppState;
use crate::handler::event_log::post_handler;
use crate::utils::logger::info;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::post;
use axum::{Router, middleware};

pub mod app_state;

pub async fn build(app_state: AppState) -> Router {
    Router::new()
        .route("/event_log", post(post_handler))
        .layer(middleware::from_fn(logger))
        .with_state(app_state)
}

async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    info(format!("{} {}", method, uri));
    next.run(req).await
}
