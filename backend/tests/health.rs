use reqwest::StatusCode;

mod helper;

#[tokio::test]
async fn return_200() {
  let base_url = helper::spawn_app().await;

  let actual = reqwest::Client::new()
    .get(&format!("{}/health", base_url))
    .send()
    .await
    .unwrap();

  assert_eq!(StatusCode::OK, actual.status());
}
