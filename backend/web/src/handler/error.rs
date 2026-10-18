use crate::utils::logger::Logger;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct Error {
    cause: Cause,
    message: String,
}

impl Error {
    pub(super) fn new(cause: Cause, message: &str) -> Self {
        Self {
            cause,
            message: message.to_owned(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self.cause {
            Cause::InternalServerError => {
                Logger::error(serde_json::to_string(&self).unwrap());
            }
            _ => {
                Logger::warn(serde_json::to_string(&self).unwrap());
            }
        };
        (self.cause.status_code(), self.message).into_response()
    }
}

#[derive(Serialize)]
pub(super) enum Cause {
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}

impl Cause {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict => StatusCode::CONFLICT,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
