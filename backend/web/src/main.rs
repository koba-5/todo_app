use sqlx::postgres::PgPoolOptions;
use std::env::var;
use std::sync::Arc;
use web::app::app_start;
use web::app::app_state::AppState;
use web::config::Config;
use web::db::Executor;

#[tokio::main]
async fn main() {
    let config = Config::new(from_env("SERVER_ADDR"), from_env("DATABASE_URL"));
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url)),
    );

    app_start(config, AppState::new(Arc::new(Executor::new(pool)))).await;
}

fn from_env(key: &str) -> String {
    var(key).unwrap_or_else(|_| panic!("no such env var: {}", key))
}
