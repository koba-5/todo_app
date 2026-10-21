use crate::utils::config::from_env;

pub(super) struct Config {
    pub(super) database_url: String,
}

impl Config {
    pub(super) fn new() -> Self {
        let database_url = from_env("DATABASE_URL");
        Self { database_url }
    }
}
