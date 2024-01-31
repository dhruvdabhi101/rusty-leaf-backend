use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header};

use super::auth::{Claims, SECRET};

pub fn create_jwt(uid: String) -> Result<String, Error> {
    let header = Header::new(Algorithm::HS512);
    let claims = Claims {
        sub: uid.to_owned(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };
    encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref())).map_err(|e| e.into())
}

pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &jsonwebtoken::Validation::new(Algorithm::HS512),
    )
    .map(|data| data.claims)
    .map_err(|e| e.into())
}
