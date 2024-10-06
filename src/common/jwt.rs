use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use bson::oid::ObjectId;
use crate::config::CONFIG;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: String,
    pub exp: usize,
}

pub fn generate_jwt(object_id: ObjectId, secret: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        + Duration::from_secs(60 * 60 * 24 * 30);

    let claims = Claims {
        id: object_id.to_hex(),
        exp: expiration.as_secs() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).unwrap()
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, Error> {
    let decoding_key = DecodingKey::from_secret(CONFIG.jwt_secret.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default())
}