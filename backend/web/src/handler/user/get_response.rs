use serde::Serialize;

#[derive(Serialize)]
pub(super) struct GetResponse {
    pub(super) name: String,
    pub(super) email: String,
    pub(super) password: String,
}
