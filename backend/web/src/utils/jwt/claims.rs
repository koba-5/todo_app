use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct Claims {
    iss: String,
    pub(super) user_id: usize,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(iss: &str, user_id: usize, iat: usize, exp: usize) -> Self {
        Self {
            iss: iss.to_string(),
            user_id,
            iat,
            exp,
        }
    }
}
