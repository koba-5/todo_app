mod utils;

use crate::utils::helper::setup;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use serde_json::{Value, json, to_vec};
use tower::Service;

#[tokio::test]
async fn post() {
    // setup
    let mut app = setup().await;

    // given
    let request = Request::post("/public/users")
        .header("content-type", "application/json")
        .body(Body::from(
            to_vec(&json!(
                {"name":"xxx", "email":"dummy@example.com", "password":"xxx"}
            ))
            .unwrap(),
        ))
        .unwrap();

    // when
    let actual = app.call(request).await.unwrap();

    // then
    assert_eq!(actual.status(), StatusCode::OK);

    let body_bytes = to_bytes(actual.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(body.get("token").is_some());
}
