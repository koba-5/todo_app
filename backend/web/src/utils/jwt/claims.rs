use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    iss: String,
    pub(crate) user_id: i64,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(iss: &str, user_id: i64, iat: usize, exp: usize) -> Self {
        Self {
            iss: iss.to_string(),
            user_id,
            iat,
            exp,
        }
    }
}
