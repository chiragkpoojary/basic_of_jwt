// src/auth.rs
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(user_id: &str) -> String {
    let expiration = 10000; // Set expiration time in seconds
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let secret = "hitherebroiamjsjfjdgbfbjdbfb";
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::new(Algorithm::HS256), &claims, &encoding_key).unwrap()
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = "hitherebroiamjsjfjdgbfbjdbfb";
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256))
        .map(|data| data.claims)
}
