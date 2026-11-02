use std::env::var;
use web::app::app_start;
use web::app::app_state::AppState;
use web::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new(from_env("SERVER_ADDR"), from_env("DATABASE_URL"));
    app_start(config, AppState::new()).await;
}

fn from_env(key: &str) -> String {
    var(key).unwrap_or_else(|_| panic!("no such env var: {}", key))
}
