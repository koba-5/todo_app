use crate::domain::Cause::{InternalServerError, ValidationError};
use crate::domain::Error;
use crate::utils::hash::hash_password;
use sqlx::FromRow;

pub mod repository;

#[derive(PartialEq, Debug, FromRow)]
pub(crate) struct User {
    pub(crate) user_id: Option<i64>,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
}

impl User {
    pub(crate) fn new(name: &str, email: &str, password: &str) -> Result<Self, Error> {
        Self::validate_name(name)?;
        Self::validate_email(email)?;
        Self::validate_password(password)?;

        Ok(Self {
            user_id: None,
            name: name.to_owned(),
            email: email.to_owned(),
            password: hash_password(password)
                .map_err(|_| Error::new(InternalServerError, "unexpected"))?,
        })
    }

    fn validate_name(name: &str) -> Result<(), Error> {
        if name.is_empty() {
            return Err(Error::new(ValidationError, "name is empty"));
        }
        if name.chars().count() > 50 {
            return Err(Error::new(ValidationError, "name is too long"));
        }
        Ok(())
    }

    fn validate_email(email: &str) -> Result<(), Error> {
        if email.is_empty() {
            return Err(Error::new(ValidationError, "email is empty"));
        }
        if email.chars().count() > 50 {
            return Err(Error::new(ValidationError, "email is too long"));
        }
        Ok(())
    }

    fn validate_password(password: &str) -> Result<(), Error> {
        if password.is_empty() {
            return Err(Error::new(ValidationError, "password is empty"));
        }
        if password.chars().count() > 50 {
            return Err(Error::new(ValidationError, "password is too long"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod new {
        use crate::domain::Error;
        use crate::domain::user::User;

        #[test]
        fn success() -> Result<(), Error> {
            // given & when
            let mut actual = User::new("山田", "dummy@example.com", "p@ssw0rd")?;

            // then
            actual.password = "dummy".to_string();
            let expected = User {
                user_id: None,
                name: "山田".to_string(),
                email: "dummy@example.com".to_string(),
                password: "dummy".to_string(),
            };
            assert_eq!(expected, actual);
            Ok(())
        }
    }
}
