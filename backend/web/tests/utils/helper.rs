use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env::var;
use std::sync::Arc;
use web::app::app_state::AppState;
use web::app::build;
use web::config::Config;
use web::repository::event_log::EventLogRepositoryImpl;
use web::repository::executor::Executor;

pub(crate) async fn setup() -> Router {
    let config = config();
    let pool = conn(&config).await;
    let app = build(app_state(Arc::new(pool))).await;

    app
}

fn config() -> Config {
    Config::new(from_env("SERVER_ADDR"), from_env("DATABASE_URL"))
}

fn from_env(key: &str) -> String {
    var(key).unwrap_or_else(|_| panic!("no such env var: {}", key))
}

async fn conn(config: &Config) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url))
}

fn app_state(pool: Arc<Pool<Postgres>>) -> AppState {
    AppState::new(
        Arc::new(Executor::new(pool)),
        Arc::new(EventLogRepositoryImpl),
    )
}
