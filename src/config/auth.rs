use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error // 3.
};

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

pub fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:3000",
        "http://0.0.0.0:8000",
    ]);

    CorsOptions { // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}