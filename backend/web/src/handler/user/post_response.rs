use serde::Serialize;

#[derive(Serialize)]
pub(super) struct PostResponse {
    pub(super) token: String,
}
