use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, errors::Error};
use chrono::{Utc, Duration};

use super::auth::{Claims, SECRET};

pub fn create_jwt(uid: &str) -> Result<String, Error> {
    let header = Header::new(Algorithm::HS512);
    let claims = Claims {
        sub: uid.to_owned(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };
    encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref())).map_err(|e| e.into())
}
