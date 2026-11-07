use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env::var;
use std::sync::Arc;
use web::app::app_state::AppState;
use web::app::build;
use web::app::jwt::Jwt;
use web::config::Config;
use web::repository::event_log::EventLogRepositoryImpl;
use web::repository::executor::Executor;

pub(crate) async fn setup() -> Router {
    let config = config();
    let pool = conn(&config).await;
    build(app_state(Arc::new(pool), &config)).await
}

fn config() -> Config {
    Config::new(
        from_env("SERVER_ADDR"),
        from_env("DATABASE_URL"),
        from_env("JWT_SECRET_KEY"),
    )
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

fn app_state(pool: Arc<Pool<Postgres>>, config: &Config) -> AppState {
    AppState::new(
        Arc::new(Jwt::new(&config.secret)),
        Arc::new(Executor::new(pool)),
        Arc::new(EventLogRepositoryImpl),
    )
}
