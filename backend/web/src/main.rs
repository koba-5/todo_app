use axum::Router;
use web::app::App;
use web::app::app_state::AppState;
use web::handler::todo::todo_routes;
use web::handler::user::user_routes;
use web::utils::jwt::Jwt;
use web::utils::tx_executor::TxExecutor;

#[tokio::main]
async fn main() {
    let app_state = AppState::new(TxExecutor::new().await, Jwt::new());

    let app = App::new(
        Router::new().route("/user", user_routes()),
        Router::new().route("/todo", todo_routes()),
        app_state,
    );
    app.start().await;
}
