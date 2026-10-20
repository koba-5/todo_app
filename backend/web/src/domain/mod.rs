pub(crate) mod user;

#[derive(Debug)]
pub(crate) struct Error {
    cause: Cause,
    pub(crate) message: String,
}

impl Error {
    pub(crate) fn new(cause: Cause, message: &str) -> Self {
        Self {
            cause,
            message: message.to_owned(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Cause {
    ValidationError,
    InternalServerError,
}
