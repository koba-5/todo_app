use serde::Serialize;

#[derive(Serialize)]
pub(super) enum Level {
    Info,
    Warn,
    Error,
}
