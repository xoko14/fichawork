use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub iat: i64,
    pub scope: String,
}

impl Claims {
    pub fn new(id: i32, scope: &str) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
            scope: scope.to_owned(),
        }
    }

    pub fn sign(self) -> Result<String, ApiError> {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(crate::JWT_SECRET.as_bytes()),
        )?)
    }
}

pub fn verify_jwt(token: &str) -> Result<Claims, ApiError> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(crate::JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
