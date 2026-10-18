use crate::domain::Cause::ValidationError;
use crate::domain::Error;

pub mod repository;

pub(crate) struct User {
    user_id: Option<i64>,
    name: String,
    email: String,
    password: String,
}

impl User {
    pub(crate) fn new(name: &str, email: &str, password: &str) -> Result<Self, Error> {
        Self::validate_name(name)?;
        Self::validate_email(email)?;
        Ok(Self {
            user_id: None,
            name: name.to_owned(),
            email: email.to_owned(),
            password: password.to_owned(),
        })
    }

    fn validate_name(name: &str) -> Result<(), Error> {
        if name.is_empty() {
            return Err(Error::new(ValidationError, "name is empty"));
        }
        if name.len() > 50 {
            return Err(Error::new(ValidationError, "name is too long"));
        }
        Ok(())
    }

    fn validate_email(email: &str) -> Result<(), Error> {
        if email.is_empty() {
            return Err(Error::new(ValidationError, "email is empty"));
        }
        if email.len() > 50 {
            return Err(Error::new(ValidationError, "email is too long"));
        }
        Ok(())
    }
}
