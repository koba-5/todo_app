use crate::app::claims::Claims;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use time::{Duration, OffsetDateTime};

pub struct Jwt {
    secret: String,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_owned(),
        }
    }

    pub(super) async fn create_token(&self, user_id: &str) -> Result<String, Error> {
        let exp = OffsetDateTime::now_utc()
            .checked_add(Duration::hours(1))
            .unwrap()
            .unix_timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret.as_bytes()),
        )
    }

    pub fn verify_jwt(&self, token: &str) -> Result<String, Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims.sub)
    }
}
