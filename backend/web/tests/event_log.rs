use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, to_vec};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env::var;
use std::sync::Arc;
use tower::Service;
use web::app::app_state::AppState;
use web::app::build;
use web::config::Config;
use web::repository::event_log::EventLogRepositoryImpl;
use web::repository::executor::Executor;

#[tokio::test]
async fn post() {
    // before
    let config = Config::new(from_env("SERVER_ADDR"), from_env("DATABASE_URL"));
    let pool = Arc::new(conn(&config).await);
    let mut app = build(app_state(pool)).await;

    // given
    let request = Request::post("/event_log")
        .header("content-type", "application/json")
        .body(
            Body::from(to_vec(&json!({"session_id":"xxx", "user_id":1, "event_type":"click", "event_name":"submit"})).unwrap())).unwrap();

    // when
    let actual = app.call(request).await.unwrap();

    // then
    assert_eq!(actual.status(), StatusCode::CREATED);
}

fn from_env(key: &str) -> String {
    var(key).unwrap_or_else(|_| panic!("no such env var: {}", key))
}

fn app_state(pool: Arc<Pool<Postgres>>) -> AppState {
    AppState::new(
        Arc::new(Executor::new(pool)),
        Arc::new(EventLogRepositoryImpl),
    )
}

async fn conn(config: &Config) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url))
}
