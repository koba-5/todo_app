use axum::Router;

use crate::app::AppState;

mod login;
mod register;

pub fn routes() -> Router<AppState> {
  Router::new().nest(
    "/auth",
    login::routes().merge(register::routes()),
  )
}
