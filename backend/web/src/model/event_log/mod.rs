pub mod repository;

pub struct EventLog {
    pub(crate) session_id: String,
    pub(crate) user_id: i64,
    pub(crate) event_type: String,
    pub(crate) event_name: String,
}

impl EventLog {
    pub(crate) fn new(
        session_id: String,
        user_id: i64,
        event_type: String,
        event_name: String,
    ) -> Self {
        Self {
            session_id,
            user_id,
            event_type,
            event_name,
        }
    }
}
