use chrono::{Duration, Utc, DateTime};
use jsonwebtoken::errors::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: String,
    exp: i64, // seconds since the epoch
}

impl Claims {
    fn new(user_id: &str) -> Self {
        Self {
            sub: user_id.to_owned(),
            exp: (Utc::now() + Duration::try_weeks(3).unwrap()).timestamp(),
        }
    }

    pub fn user_id(&self) -> &str {
        &self.sub
    }

    pub fn with_expiration(mut self, exp: DateTime<Utc>) -> Self {
        self.exp = exp.timestamp();
        self
    }
}

pub fn _try_encode_token(secret: &str, sub: &str) -> Result<String> {
    encode(
        &Header::default(),
        &Claims::new(sub),
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn try_encode_token_exp(secret: &str, sub: &str, exp: DateTime<Utc>) -> Result<String> {
    encode(
        &Header::default(),
        &Claims::new(sub).with_expiration(exp),
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_token(secret: &str, token: &str) -> Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_token() {
        let sub = "test";
        let token = _try_encode_token("secret", sub).unwrap();
        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        );
        if let Err(e) = &decoded {
            println!("decode err: {}", e);
        }

        assert!(decoded.is_ok());
    }
}