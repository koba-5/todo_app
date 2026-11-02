use crate::app::app_state::AppState;
use crate::handler::Error;
use crate::handler::event_log::post_request::PostRequest;
use crate::model::event_log::EventLog;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

mod post_request;

pub(crate) async fn post_handler(
    State(app_state): State<AppState>,
    Json(request): Json<PostRequest>,
) -> Result<impl IntoResponse, Error> {
    let (executor, repository) = (app_state.executor, app_state.event_log_repository);

    executor
        .with_commit(async move |conn| {
            repository
                .create(
                    conn,
                    EventLog::new(
                        request.session_id,
                        request.user_id,
                        request.event_type,
                        request.event_name,
                    ),
                )
                .await
        })
        .await
        .map_err(Error::from)?;

    Ok((StatusCode::CREATED, ()))
}
