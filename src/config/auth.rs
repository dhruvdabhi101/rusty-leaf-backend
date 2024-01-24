use rocket::{
    outcome::Outcome,
    http::Status,
    request::{self, FromRequest, Request},
};

use super::jwt::decode_jwt;

pub const SECRET: &str = "secrets";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn hash_password(password: String) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}

// pub struct Token(pub String);
// 
// #[derive(Debug)]
// enum ApiTokenError {
//     Missing,
//     Invalid,
// }
// 
// #[shuttle_runtime::async_trait]
// impl<'a, 'r> FromRequest<'a> for Token {
//     type Error = ApiTokenError;
// 
//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//         let token = request.headers().get_one("Authorization");
//         match token {
//             Some(token) => {
//                 let token_str = decode_jwt(token).unwrap();
//                 Outcome::Success(Token(token_str.sub))
//             }
//             None => Outcome::Error((Status::Unauthorized, ApiTokenError::Missing)),
//         }
//     }
// }
