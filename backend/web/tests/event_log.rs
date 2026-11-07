mod utils;

use crate::utils::helper::setup;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, to_vec};
use tower::Service;

#[tokio::test]
async fn post() {
    // setup
    let mut app = setup().await;

    // given
    let request = Request::post("/public/event_log")
        .header("content-type", "application/json")
        .body(Body::from(
            to_vec(&json!(
                {"session_id":"xxx", "user_id":1, "event_type":"click", "event_name":"submit"}
            ))
            .unwrap(),
        ))
        .unwrap();

    // when
    let actual = app.call(request).await.unwrap();

    // then
    assert_eq!(actual.status(), StatusCode::CREATED);
}
