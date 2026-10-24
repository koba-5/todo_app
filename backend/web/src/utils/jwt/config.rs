use crate::utils::config::from_env;

pub(super) struct Config {
    pub(super) secret_key: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            secret_key: from_env("JWT_SECRET_KEY"),
        }
    }
}
