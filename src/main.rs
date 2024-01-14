pub mod models;
pub mod api;
pub mod repository;
use api::user_api::{create_user, get_user};
use repository::mongodb_repo::MongoRepo;
use rocket::{get, routes, http::Status, serde::json::Json};

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongodb")))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let db = MongoRepo::init();
    let rocket = rocket::build().manage(db).mount("/", routes![index, create_user, get_user]);
    Ok(rocket.into())
}
