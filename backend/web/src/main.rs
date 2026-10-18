use web::app::App;
use web::handler::todo::todo_routes;
use web::handler::user::user_routes;

#[tokio::main]
async fn main() {
    let app = App::new(|r| {
        r.route("/user", user_routes())
            .route("/todo", todo_routes())
    });
    app.start().await;
}
