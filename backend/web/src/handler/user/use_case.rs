use crate::domain::user::User;
use crate::domain::user::repository::UserRepository;
use crate::handler::error::Cause::{BadRequest, InternalServerError};
use crate::handler::error::Error;
use crate::handler::user::get_response::GetResponse;
use crate::handler::user::post_request::PostRequest;
use crate::utils::tx_executor::TxExecutor;
use std::sync::Arc;

pub(super) struct UserUseCase<R: UserRepository> {
    tx_executor: Arc<TxExecutor>,
    user_repository: R,
}

impl<R: UserRepository> UserUseCase<R> {
    pub(super) fn new(tx_executor: Arc<TxExecutor>, user_repository: R) -> Self {
        Self {
            tx_executor,
            user_repository,
        }
    }

    pub(super) async fn get(&self, user_id: i64) -> Result<GetResponse, Error> {
        let db_result = self
            .tx_executor
            .execute_read_only(async |tx| Ok(self.user_repository.get_by_id(tx, user_id).await?))
            .await
            .map_err(|_| Error::new(InternalServerError, "internal server error"))?;

        let response = GetResponse {
            name: db_result.name,
            email: db_result.email,
            password: db_result.password,
        };

        Ok(response)
    }

    pub(super) async fn post(&self, request: &PostRequest) -> Result<(), Error> {
        let user = User::new(&request.name, &request.email, &request.password)
            .map_err(|e| Error::new(BadRequest, &e.message))?;

        self.tx_executor
            .execute(async |tx| Ok(self.user_repository.save(tx, user).await?))
            .await
            .map_err(|_| Error::new(InternalServerError, "internal server error"))?;

        Ok(())
    }
}
