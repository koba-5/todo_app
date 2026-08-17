use std::sync::{Arc, Mutex};

use axum::Router;
use oracle::Connection;
use tower_http::trace::TraceLayer;

use crate::handler;

#[derive(Clone)]
pub struct AppState {
  pub conn: Arc<Mutex<Connection>>,
}

pub fn create_app(state: AppState) -> Router {
  Router::new()
    .merge(handler::health::routes())
    .merge(handler::auth::routes())
    .layer(TraceLayer::new_for_http())
    .with_state(state)
}
