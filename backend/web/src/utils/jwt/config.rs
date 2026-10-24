use crate::utils::config::from_env;

pub(super) struct Config {
    pub(super) secret_key: String,
}

impl Config {
    pub(super) fn new() -> Self {
        let secret_key = from_env("JWT_SECRET_KEY");
        Self { secret_key }
    }
}
