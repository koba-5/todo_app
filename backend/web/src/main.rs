use axum::{Router, serve};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env::var;
use std::sync::Arc;
use tokio::net::TcpListener;
use web::app::app_state::AppState;
use web::app::build;
use web::app::jwt::Jwt;
use web::config::Config;
use web::repository::event_log::EventLogRepositoryImpl;
use web::repository::executor::Executor;
use web::utils::logger::info;

#[tokio::main]
async fn main() {
    let config = Config::new(
        from_env("SERVER_ADDR"),
        from_env("DATABASE_URL"),
        from_env("JWT_SECRET_KEY"),
    );
    let pool = Arc::new(conn(&config).await);

    let app = build(app_state(pool, &config)).await;
    let listener = bind(&config).await;
    info(format!("Listening on {}", &config.server_addr));
    start(listener, app).await;
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

async fn bind(config: &Config) -> TcpListener {
    TcpListener::bind(&config.server_addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind to {}", &config.server_addr))
}

async fn start(listener: TcpListener, app: Router) {
    serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("server failed to start"));
}
