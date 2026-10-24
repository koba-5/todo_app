use std::env::var;

pub(crate) fn from_env(key: &str) -> String {
    var(key).unwrap_or_else(|_| panic!("{} is not set", key))
}
