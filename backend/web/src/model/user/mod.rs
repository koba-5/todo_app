use crate::model::Error;
use crate::model::Error::Unexpected;
use crate::utils::password::hash_password;

pub mod repository;

pub struct User {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password_hash: String,
}

impl User {
    pub(crate) fn new(name: String, email: String, password: String) -> Result<Self, Error> {
        let password_hash = hash_password(&password).map_err(|e| Unexpected(e.to_string()))?;
        Ok(Self {
            name,
            email,
            password_hash,
        })
    }
}
