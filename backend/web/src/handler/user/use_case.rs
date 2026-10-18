use crate::handler::error::Error;

pub(super) struct UserUseCase;

impl UserUseCase {
    pub(super) fn new() -> Self {
        Self
    }

    pub(super) async fn get(&self) -> Result<String, Error> {
        Ok("Hello World!".to_string())
    }
}
