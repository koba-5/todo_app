use crate::model;
use crate::model::Error::Unexpected;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub(crate) mod event_log;

pub(crate) enum Error {
    InternalServerError(String),
}

impl Error {
    pub(super) fn info(self) -> (StatusCode, String) {
        match self {
            Self::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        }
    }
}

impl From<model::Error> for Error {
    fn from(e: model::Error) -> Self {
        match e {
            Unexpected(msg) => Self::InternalServerError(msg),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.info().into_response()
    }
}
