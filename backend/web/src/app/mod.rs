use crate::app::app_state::AppState;
use crate::config::Config;
use crate::handler::event_log::post_handler;
use axum::routing::post;
use axum::{Router, serve};
use tokio::net::TcpListener;

pub mod app_state;

pub async fn app_start(config: Config, app_state: AppState) {
    let app = Router::new()
        .route("/event_log", post(post_handler))
        .with_state(app_state);
    let listener = TcpListener::bind(&config.server_addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind to {}", &config.server_addr));

    println!("Listening on {}", &config.server_addr);

    serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("failed to serve connection from {}", &config.server_addr));
}
