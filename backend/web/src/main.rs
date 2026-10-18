use axum::routing::get;
use web::app::App;

#[tokio::main]
async fn main() {
    let app = App::new(|r| r.route("/", get("Hello")));
    app.start().await;
}
