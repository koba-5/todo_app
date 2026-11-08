use crate::app::app_state::AppState;
use crate::handler::Error;
use crate::handler::Error::InternalServerError;
use crate::handler::user::post_request::PostRequest;
use crate::handler::user::post_response::PostResponse;
use crate::model::user::User;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

mod post_request;
mod post_response;

pub(crate) async fn post_handler(
    State(app_state): State<AppState>,
    Json(request): Json<PostRequest>,
) -> Result<impl IntoResponse, Error> {
    let (jwt, executor, repository) =
        (app_state.jwt, app_state.executor, app_state.user_repository);

    let user_id = executor
        .with_commit(async move |conn| {
            Ok(repository
                .create(
                    conn,
                    User::new(request.name, request.email, request.password)?,
                )
                .await?)
        })
        .await
        .map_err(Error::from)?;

    let token = jwt
        .create_token(user_id)
        .await
        .map_err(|e| InternalServerError(e.to_string()))?;

    Ok((StatusCode::OK, Json(PostResponse::new(token))))
}
