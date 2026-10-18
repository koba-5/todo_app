use crate::utils::config::from_env;
use crate::utils::logger::Logger;
use axum::{Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct App {
    router: Router,
    addr: SocketAddr,
}

impl App {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Router) -> Router,
    {
        let ip = from_env("SERVER_IP");
        let port = from_env("SERVER_PORT");
        let addr = format!("{}:{}", ip, port).parse::<SocketAddr>().unwrap();

        let router = f(Router::new());

        Self { router, addr }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(self.addr).await.unwrap();
        Logger::info(format!("Listening on {}", self.addr));
        serve(listener, self.router.clone()).await.unwrap()
    }
}
