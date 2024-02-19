pub mod api;
pub mod config;
pub mod models;
pub mod repository;
use api::{
    page_api::{create_page, delete_page, get_all, get_page, get_page_by_id, update_page},
    user_api::{create_user, get_user, login},
};
use config::auth::make_cors;
use repository::mongodb_repo::MongoRepo;
use rocket::{
    fairing::{Fairing, Info, Kind},
    get,
    http::{Header, Status},
    routes,
    serde::json::Json,
    Request, Response, Rocket,
};

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongodb")))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let db: MongoRepo = MongoRepo::init();
    let rocket: Rocket<rocket::Build> = rocket::build()
        .attach(make_cors())
        .manage(db)
        .mount("/", routes![create_user, get_user, login, index])
        .mount(
            "/pages",
            routes![create_page, get_page, update_page, delete_page, get_all, get_page_by_id],
        );
    Ok(rocket.into())
}

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
