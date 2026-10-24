use crate::utils::jwt::claims::Claims;
use crate::utils::jwt::config::Config;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};

mod claims;
mod config;

pub(crate) struct Jwt {
    secret_key: String,
}

impl Jwt {
    pub(crate) fn new() -> Self {
        let config = Config::new();
        Self {
            secret_key: config.secret_key,
        }
    }

    pub(crate) fn create(&self, user_id: i64) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        )
    }

    pub(crate) fn verify(&self, token: &str) -> Result<i64, jsonwebtoken::errors::Error> {
        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims.user_id)
    }
}

#[cfg(test)]
mod tests {
    mod success {
        use crate::utils::jwt::Jwt;

        #[test]
        fn success() -> Result<(), jsonwebtoken::errors::Error> {
            // given
            let jwt = Jwt::new();
            let token = jwt.create(99999)?;

            // when
            let actual = jwt.verify(&token)?;

            // then
            assert_eq!(actual, 99999);
            Ok(())
        }
    }
}
