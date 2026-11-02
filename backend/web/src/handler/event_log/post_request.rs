use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PostRequest {
    pub(crate) session_id: String,
    pub(crate) user_id: i64,
    pub(crate) event_type: String,
    pub(crate) event_name: String,
}
