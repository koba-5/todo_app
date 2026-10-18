use crate::utils::logger::level::Level;
use crate::utils::logger::log::Log;
use crate::utils::time::now;
use std::fmt::Display;

mod level;
mod log;

pub(crate) struct Logger;

impl Logger {
    pub(crate) fn info<T: Display>(output: T) {
        let line = Self::convert_to_line(Level::Info, output);
        println!("\x1b[32m{}\x1b[0m", line);
    }

    pub(crate) fn warn<T: Display>(output: T) {
        let line = Self::convert_to_line(Level::Warn, output);
        println!("\x1b[33m{}\x1b[0m", line);
    }

    pub(crate) fn error<T: Display>(output: T) {
        let line = Self::convert_to_line(Level::Error, output);
        eprintln!("\x1b[31m{}\x1b[0m", line);
    }

    fn convert_to_line<T: Display>(level: Level, output: T) -> String {
        let log = Log::new(level, now().to_string(), output.to_string());
        serde_json::to_string(&log).unwrap()
    }
}
