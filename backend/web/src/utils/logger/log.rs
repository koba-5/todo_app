use crate::utils::logger::level::Level;
use crate::utils::time::now;
use serde::Serialize;
use std::fmt::Display;
use time::OffsetDateTime;

#[derive(Serialize)]
pub(super) struct Log {
    level: Level,
    timestamp: String,
    output: String,
}

impl Log {
    pub(super) fn new<T: Display>(level: Level, output: T) -> Self {
        Self {
            level,
            timestamp: now().to_string(),
            output: output.to_string(),
        }
    }
}
