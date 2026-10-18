use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct PostRequest {
    pub(super) name: String,
    pub(super) email: String,
    pub(super) password: String,
}
