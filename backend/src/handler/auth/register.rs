use axum::{
  Json, Router, extract::State, routing,
};
use reqwest::StatusCode;
use web::models::{
  AuthLoginPost200Response,
  AuthRegisterPostRequest,
};

use crate::app::AppState;

pub fn routes() -> Router<AppState> {
  Router::new().route(
    "/register",
    routing::post(post_handler),
  )
}

async fn post_handler(
  State(state): State<AppState>,
  Json(payload): Json<AuthRegisterPostRequest>,
) -> Result<
  Json<AuthLoginPost200Response>,
  StatusCode,
> {
  let conn = state.conn.lock().unwrap();

  let result = conn.execute(
    r#"INSERT INTO "USER" (EMAIL, PASSWORD) VALUES (:1, :2)"#,
    &[&payload.email, &payload.password],
  );

  match result {
    Ok(_) => Ok(Json(AuthLoginPost200Response {
      access_token: "dummy_access_token"
        .to_string(),
    })),
    Err(_) => {
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
