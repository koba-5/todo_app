use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct Claims {
    pub(super) sub: i64,
    pub(super) exp: usize,
}
