use crate::utils::jwt::claims::Claims;
use crate::utils::jwt::config::Config;
use crate::utils::time::now;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use time::Duration;

mod claims;
mod config;

struct Jwt {
    secret_key: String,
}

impl Jwt {
    pub fn new() -> Self {
        let config = Config::new();
        Self {
            secret_key: config.secret_key,
        }
    }

    pub(crate) fn create(&self, user_id: usize) -> Result<String, jsonwebtoken::errors::Error> {
        let now = now();
        let iat = now.unix_timestamp() as usize;
        let exp = now
            .checked_add(Duration::minutes(5))
            .unwrap()
            .unix_timestamp() as usize;
        let claims = Claims::new("todo_app", user_id, iat, exp);

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        )
    }

    pub(crate) fn verify(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
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
            let claims = jwt.verify(&token)?;

            // then
            assert_eq!(claims.user_id, 99999);
            Ok(())
        }
    }
}
