use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct Claims {
    pub(super) sub: String,
    pub(super) exp: usize,
}
