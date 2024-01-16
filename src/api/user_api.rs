use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;
use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::{
    models::user_model::{LoginResponse, LoginUser, User, UserFromMongo},
    repository::mongodb_repo::MongoRepo, config::jwt::create_jwt,
};

#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        username: new_user.username.clone(),
        password: new_user.password.clone(),
        email: new_user.email.clone(),
        name: new_user.name.clone(),
    };
    let user_details = db.create_user(data);
    match user_details {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: &str) -> Result<Json<UserFromMongo>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let user_details = db.get_user(&id);
    match user_details {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[post("/login", data = "<user>")]
pub fn login(db: &State<MongoRepo>, user: Json<LoginUser>) -> Result<Json<LoginResponse>, Status> {
    // get user details from db
    let user_details: Result<UserFromMongo, Error> = db.login(&user.username, &user.password);

    // if user not found return 404
    if user_details.is_err() {
        return Err(Status::InternalServerError);
    } else {
        // if user found return jwt token
        let user: UserFromMongo = user_details.unwrap();
        Ok(Json(LoginResponse {
            token: create_jwt(user._id.to_hex()).unwrap(),
        }))
    }
}
