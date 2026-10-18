use crate::handler::error::Error;
use crate::handler::user::use_case::UserUseCase;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub(super) struct UserHandler;

impl UserHandler {
    pub(super) async fn get_handler() -> Result<impl IntoResponse, Error> {
        let response = UserUseCase::new().get().await?;
        Ok((StatusCode::OK, response))
    }
}
