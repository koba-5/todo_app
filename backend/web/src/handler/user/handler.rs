use crate::handler::error::Error;
use crate::handler::user::post_request::PostRequest;
use crate::handler::user::use_case::UserUseCase;
use crate::repository::user::real::UserRepositoryReal;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub(super) struct UserHandler;

impl UserHandler {
    pub(super) async fn get_handler() -> Result<impl IntoResponse, Error> {
        let user_repository = UserRepositoryReal::new();
        let response = UserUseCase::new(user_repository).get().await?;
        Ok((StatusCode::OK, response))
    }

    pub(super) async fn post_handler(
        Json(request): Json<PostRequest>,
    ) -> Result<impl IntoResponse, Error> {
        let user_repository = UserRepositoryReal::new();
        let response = UserUseCase::new(user_repository).post(&request).await?;
        Ok((StatusCode::OK, response))
    }
}
