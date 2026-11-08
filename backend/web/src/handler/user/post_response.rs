use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct PostResponse {
    pub(crate) token: String,
}

impl PostResponse {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }
}
