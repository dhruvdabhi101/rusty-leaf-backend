pub mod api;
pub mod config;
pub mod models;
pub mod repository;
use api::{user_api::{create_user, get_user, login}, page_api::create_page};
use repository::mongodb_repo::MongoRepo;
use rocket::{get, http::Status, routes, serde::json::Json, Rocket};

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongodb")))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let db: MongoRepo = MongoRepo::init();
    let rocket: Rocket<rocket::Build> = rocket::build()
        .manage(db)
        .mount("/", routes![index, create_user, get_user, login])
        .mount("/pages", routes![create_page]);
    Ok(rocket.into())
}
