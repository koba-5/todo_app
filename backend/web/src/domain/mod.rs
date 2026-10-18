pub(crate) mod user;

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

pub(crate) enum Cause {
    ValidationError,
}
