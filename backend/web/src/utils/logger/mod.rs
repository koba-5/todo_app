use crate::utils::logger::level::Level;
use crate::utils::logger::log::Log;
use std::fmt::Display;

mod level;
mod log;

pub(crate) struct Logger;

impl Logger {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn info<T: Display>(output: T) {
        let log = Log::new(Level::Info, output);
        let line = Self::convert_to_line(log);
        println!("{}", line);
    }

    pub(crate) fn warn<T: Display>(output: T) {
        let log = Log::new(Level::Info, output);
        let line = Self::convert_to_line(log);
        println!("{}", line);
    }

    pub(crate) fn error<T: Display>(output: T) {
        let log = Log::new(Level::Info, output);
        let line = Self::convert_to_line(log);
        println!("{}", line);
    }

    fn convert_to_line(log: Log) -> String {
        serde_json::to_string(&log).unwrap()
    }
}
