use axum::{
  Json, Router, extract::State, routing,
};
use reqwest::StatusCode;
use web::models::{
  AuthLoginPost200Response, AuthLoginPostRequest,
};

use crate::app::AppState;

pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/login", routing::post(post_handler))
}

async fn post_handler(
  State(state): State<AppState>,
  Json(payload): Json<AuthLoginPostRequest>,
) -> Result<
  Json<AuthLoginPost200Response>,
  StatusCode,
> {
  let conn = state.conn.lock().unwrap();

  let result = conn.query_row_as::<i64>(
    r#"SELECT USER_ID FROM \"USER\" WHERE email = :1 AND password = :2"#,
    &[&payload.email, &payload.password],
  );

  match result {
    Ok(_) => Ok(Json(AuthLoginPost200Response {
      access_token: "dummy_access_token"
        .to_string(),
    })),
    Err(e)
      if e.kind()
        == oracle::ErrorKind::NoDataFound =>
    {
      Err(StatusCode::UNAUTHORIZED)
    }
    Err(e) => {
      tracing::error!("Database error: {:?}", e);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
