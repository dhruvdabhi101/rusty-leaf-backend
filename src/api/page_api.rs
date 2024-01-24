use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;
use rocket::{get, http::Status, post, serde::json::Json, State};
use rocket_authorization::oauth::OAuth;
use rocket_authorization::{AuthError, Credential};

use crate::config::jwt::decode_jwt;
use crate::{
    models::{user_model::{LoginResponse, LoginUser, User, UserFromMongo}, page_model::{PageCreateRequest, PageCreateResponse, Page}},
    repository::mongodb_repo::MongoRepo,
};

#[post("/create-page", data="<new_page>")]
pub fn create_page(auth: Credential<OAuth>, db: &State<MongoRepo>, new_page: Json<PageCreateRequest>) -> Result<Json<PageCreateResponse>, Status> {
    // get user from jwt token 
    let token = &auth.token;
    println!("{:?}", token);

    let new_page = PageCreateRequest { 
        title: new_page.title.clone(),
        content: new_page.content.clone(),
        slug: new_page.slug.clone(),
        published: new_page.published.clone(),
    };

    let page: PageCreateResponse = db.create_page(new_page, token).unwrap();
    Ok(Json(page))
}

