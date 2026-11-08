use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PostRequest {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
}
