use crate::app::claims::Claims;
use crate::config::Config;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use time::{Duration, OffsetDateTime};

pub struct Jwt {
    secret: String,
}

impl Jwt {
    pub fn new(config: &Config) -> Self {
        Self {
            secret: config.secret.to_owned(),
        }
    }

    pub(crate) async fn create_token(&self, user_id: i64) -> Result<String, Error> {
        let exp = OffsetDateTime::now_utc()
            .checked_add(Duration::hours(1))
            .unwrap()
            .unix_timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret.as_bytes()),
        )
    }

    pub(super) fn verify_jwt(&self, token: &str) -> Result<i64, Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims.sub)
    }
}
