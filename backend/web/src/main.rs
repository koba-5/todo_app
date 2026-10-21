use axum::Router;
use std::sync::Arc;
use web::app::App;
use web::handler::todo::todo_routes;
use web::handler::user::user_routes;
use web::utils::tx_executor::TxExecutor;

#[tokio::main]
async fn main() {
    let tx_executor = Arc::new(TxExecutor::new().await);
    let app = App::new(
        Router::new().route("/user", user_routes()),
        Router::new().route("/todo", todo_routes()),
        tx_executor,
    );
    app.start().await;
}
