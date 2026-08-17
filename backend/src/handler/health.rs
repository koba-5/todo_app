use axum::{
  Router, extract::State, http::StatusCode,
  routing,
};

use crate::app::AppState;

pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/health", routing::get(get_handler))
}

async fn get_handler(
  State(state): State<AppState>,
) -> StatusCode {
  let conn = state.conn.lock().unwrap();

  match conn.ping() {
    Ok(_) => StatusCode::OK,
    _ => StatusCode::SERVICE_UNAVAILABLE,
  }
}
