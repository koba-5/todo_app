use std::sync::Arc;
use std::sync::Mutex;

use backend::app;
use backend::config::Config;
use backend::db::Db;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let config = Config::new();
  let db = Db::new(
    config.username,
    config.password,
    config.db_url,
  );
  let app = app::create_app(app::AppState {
    conn: Arc::new(Mutex::new(db.conn)),
  });

  let listener = tokio::net::TcpListener::bind(
    format!("127.0.0.1:{}", config.server_port),
  )
  .await
  .expect("failed to bind");

  tracing::info!(
    "Listening on http://{}",
    listener.local_addr().unwrap()
  );
  axum::serve(listener, app)
    .await
    .expect("server error");
}
