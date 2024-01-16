use mongodb::results::InsertOneResult;
use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::{models::user_model::{User, LoginUser, LoginResponse}, repository::mongodb_repo::MongoRepo};

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
pub fn get_user(db: &State<MongoRepo>, path: &str) -> Result<Json<User>, Status> {
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
    // use jwt token here
    // extract the token from the header
    let user_details = db.login(&user.username, &user.password);
    if user_details.is_err() {
        return Err(Status::InternalServerError);
    } else {
        let user = user_details.unwrap();
        Ok(Json(LoginResponse {
            token: "".to_string(),
            user: User {
                username: user.username,
                password: user.password,
                email: user.email,
                name: user.name,
            },
            
        }))

    }

}
