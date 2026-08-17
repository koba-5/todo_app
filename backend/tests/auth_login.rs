use reqwest::StatusCode;

mod helper;

#[tokio::test]
async fn return_401() {
  let base_url = helper::spawn_app().await;

  // when
  let actual = reqwest::Client::new()
    .post(&format!("{}/auth/login", base_url))
    .json(&serde_json::json!({
        "email": "dummy@example.com",
        "password": "dummy"
    }))
    .send()
    .await
    .unwrap();

  // then
  assert_eq!(
    StatusCode::UNAUTHORIZED,
    actual.status()
  );
}
