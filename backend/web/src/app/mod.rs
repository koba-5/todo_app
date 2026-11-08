use crate::app::app_state::AppState;
use crate::handler::event_log;
use crate::utils::logger::info;
use axum::extract::{Request, State};
use axum::http::{StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Router, middleware};

pub mod app_state;
pub(super) mod claims;
pub mod jwt;

pub async fn build(app_state: AppState) -> Router {
    let public_router = Router::new().route("/event_log", post(event_log::post_handler));

    let private_router = Router::new()
        .route("/users", post(event_log::post_handler))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth));

    Router::new()
        .nest("/public", public_router)
        .nest("/private", private_router)
        .layer(middleware::from_fn(logger))
        .with_state(app_state)
}

async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    info(format!("{} {}", method, uri));
    next.run(req).await
}

async fn auth(State(app_state): State<AppState>, mut req: Request, next: Next) -> Response {
    let jwt = app_state.jwt;

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok());

    let token = match auth_header {
        Some(header_val) if header_val.starts_with("Bearer ") => &header_val[7..],
        _ => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let claims = match jwt.verify_jwt(token) {
        Ok(c) => c,
        Err(_) => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    req.extensions_mut().insert(claims);
    next.run(req).await
}
