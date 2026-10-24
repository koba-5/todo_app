use crate::utils::jwt::Jwt;
use crate::utils::tx_executor::TxExecutor;

#[derive(Clone)]
pub struct AppState {
    pub(crate) tx_executor: TxExecutor,
    pub(crate) jwt: Jwt,
}

impl AppState {
    pub fn new(tx_executor: TxExecutor, jwt: Jwt) -> Self {
        Self { tx_executor, jwt }
    }
}
