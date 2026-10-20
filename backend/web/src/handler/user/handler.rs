use crate::handler::error::Cause::BadRequest;
use crate::handler::error::Error;
use crate::handler::user::post_request::PostRequest;
use crate::handler::user::use_case::UserUseCase;
use crate::repository::user::real::UserRepositoryReal;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;

pub(super) struct UserHandler;

impl UserHandler {
    pub(super) async fn get_handler(
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<impl IntoResponse, Error> {
        let user_id = params
            .get("user_id")
            .ok_or_else(|| Error::new(BadRequest, "user_id is required"))?
            .parse::<i64>()
            .map_err(|_| Error::new(BadRequest, "user_id is required"))?;

        let user_repository = UserRepositoryReal::new();
        let response = UserUseCase::new(user_repository).get(user_id).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    pub(super) async fn post_handler(
        Json(request): Json<PostRequest>,
    ) -> Result<impl IntoResponse, Error> {
        let user_repository = UserRepositoryReal::new();
        let response = UserUseCase::new(user_repository).post(&request).await?;
        Ok((StatusCode::CREATED, response))
    }
}
