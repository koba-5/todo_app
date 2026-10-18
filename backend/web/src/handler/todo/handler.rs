use crate::handler::error::Error;
use crate::handler::todo::use_case::TodoUseCase;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub(super) struct TodoHandler;

impl TodoHandler {
    pub(super) async fn get_handler() -> Result<impl IntoResponse, Error> {
        let response = TodoUseCase::new().get().await?;
        Ok((StatusCode::OK, response))
    }
}
