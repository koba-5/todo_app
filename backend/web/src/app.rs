use crate::utils::config::from_env;
use crate::utils::logger::Logger;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Router, middleware, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct App {
    public_router: Router,
    protected_router: Router,
    addr: SocketAddr,
}

impl App {
    pub fn new(public_router: Router, protected_router: Router) -> Self {
        let ip = from_env("SERVER_IP");
        let port = from_env("SERVER_PORT");
        let addr = format!("{}:{}", ip, port).parse::<SocketAddr>().unwrap();

        Self {
            public_router,
            protected_router,
            addr,
        }
    }

    pub async fn start(self) {
        let app = Router::new()
            .merge(Router::new().nest("/public", self.public_router))
            .merge(
                Router::new()
                    .nest("/public", self.protected_router)
                    .layer(middleware::from_fn(Self::auth)),
            )
            .layer(middleware::from_fn(Self::logging));

        let listener = TcpListener::bind(self.addr).await.unwrap();
        Logger::info(format!("Listening on {}", self.addr));
        serve(listener, app).await.unwrap()
    }

    async fn logging(req: Request<Body>, next: Next) -> Response {
        Logger::info(format!("{:?}", req));
        next.run(req).await
    }

    async fn auth(req: Request<Body>, next: Next) -> Response {
        Logger::info(format!("{:?}", req));
        next.run(req).await
    }
}
