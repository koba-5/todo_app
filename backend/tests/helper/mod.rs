use std::sync::{Arc, Mutex};

use backend::{app, config::Config, db::Db};

pub async fn spawn_app() -> String {
  let config = Config::new();
  let db = Db::new(
    config.username,
    config.password,
    config.db_url,
  );
  let app = app::create_app(app::AppState {
    conn: Arc::new(Mutex::new(db.conn)),
  });
  let listener =
    tokio::net::TcpListener::bind("127.0.0.1:0")
      .await
      .unwrap();
  let addr = listener.local_addr().unwrap();

  tokio::spawn(async move {
    axum::serve(listener, app).await.unwrap();
  });

  format!("http://{}", addr)
}
