use crate::utils::config::from_env;
use crate::utils::logger::Logger;
use crate::utils::tx_executor::TxExecutor;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Router, middleware, serve};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct App {
    router: Router,
    addr: SocketAddr,
}

impl App {
    pub fn new(
        public_router: Router<Arc<TxExecutor>>,
        protected_router: Router<Arc<TxExecutor>>,
        tx_executor: Arc<TxExecutor>,
    ) -> Self {
        let ip = from_env("SERVER_IP");
        let port = from_env("SERVER_PORT");
        let addr = format!("{}:{}", ip, port).parse::<SocketAddr>().unwrap();

        let router = Router::new()
            .nest("/public", public_router)
            .nest(
                "/protected",
                protected_router.layer(middleware::from_fn(Self::auth)),
            )
            .layer(middleware::from_fn(Self::logging))
            .with_state(tx_executor);

        Self { router, addr }
    }

    pub async fn start(self) {
        let listener = TcpListener::bind(self.addr).await.unwrap();
        Logger::info(format!("Listening on {}", self.addr));
        serve(listener, self.router).await.unwrap()
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
