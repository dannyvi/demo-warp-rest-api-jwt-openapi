use crate::errors::ApiError;
use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    pub fn new(id: String, permissions: Vec<String>, exp: i64) -> Self {
        Self {
            id,
            permissions,
            exp: (Utc::now() + Duration::hours(exp)).timestamp(),
        }
    }
}

// type DateTimeUtc = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Jwt {
    secret: String,
    expire: i64,
}

impl Jwt {
    pub fn new(secret: &String, expire: &i64) -> Self {
        Self { secret: secret.to_string(), expire: *expire }
    }

    pub fn new_claim(&self, id: String, permissions: Vec<String>) -> anyhow::Result<Claims> {
        Ok(Claims::new(id, permissions, self.expire))
    }

    pub fn encode(&self, claims: Claims) -> Result<String, ApiError> {
        let encoding_key = EncodingKey::from_secret(&self.secret.as_bytes());
        jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
            .map_err(|e| ApiError::Unauthorized(e.to_string()))
    }

        pub fn decode(&self, token: &str) -> Result<Claims, ApiError> {
        let decoding_key = DecodingKey::from_secret(&self.secret.as_bytes());
        jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| ApiError::Unauthorized(e.to_string()))
    }

}
