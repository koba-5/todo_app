use crate::utils::time::now;
use serde::{Deserialize, Serialize};
use time::Duration;

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    iss: String,
    pub(crate) user_id: i64,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(user_id: i64) -> Self {
        let now = now();
        let iat = now.unix_timestamp() as usize;
        let exp = now
            .checked_add(Duration::minutes(5))
            .unwrap()
            .unix_timestamp() as usize;

        Self {
            iss: "todo_app".to_string(),
            user_id,
            iat,
            exp,
        }
    }
}
