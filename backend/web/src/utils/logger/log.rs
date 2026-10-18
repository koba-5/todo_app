use crate::utils::logger::level::Level;
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct Log {
    level: Level,
    timestamp: String,
    output: String,
}

impl Log {
    pub(super) fn new(level: Level, timestamp: String, output: String) -> Self {
        Self {
            level,
            timestamp,
            output,
        }
    }
}
