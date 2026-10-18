use crate::domain::user::User;
use crate::domain::user::repository::UserRepository;
use crate::handler::error::Cause::{BadRequest, InternalServerError};
use crate::handler::error::Error;
use crate::handler::user::post_request::PostRequest;
use crate::utils::tx_manager::TxManager;

pub(super) struct UserUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserUseCase<R> {
    pub(super) fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub(super) async fn get(&self) -> Result<String, Error> {
        Ok("Hello World!".to_string())
    }

    pub(super) async fn post(&self, request: &PostRequest) -> Result<(), Error> {
        let user = User::new(&request.name, &request.email, &request.password)
            .map_err(|e| Error::new(BadRequest, &e.message))?;

        let tx_manager = TxManager::new().await;
        tx_manager
            .execute(async |tx| Ok(self.user_repository.save(tx, user).await?))
            .await
            .map_err(|_| Error::new(InternalServerError, "internal server error"))?;

        Ok(())
    }
}
