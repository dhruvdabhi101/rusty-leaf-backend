use mongodb::results::DeleteResult;
use rocket::{delete, get, http::Status, post, put, serde::json::Json, State};
use rocket_authorization::oauth::OAuth;
use rocket_authorization::Credential;

use crate::{
    models::page_model::{
        Page, PageCreateRequest, PageCreateResponse, PageUpdateRequest, PageUpdateResponse,
    },
    repository::mongodb_repo::MongoRepo,
};

#[post("/create-page", data = "<new_page>")]
pub fn create_page(
    auth: Credential<OAuth>,
    db: &State<MongoRepo>,
    new_page: Json<PageCreateRequest>,
) -> Result<Json<PageCreateResponse>, Status> {
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

#[get("/get-page/<username>/<slug>")]
pub fn get_page(db: &State<MongoRepo>, slug: &str, username: &str) -> Result<Json<Page>, Status> {
    let page: Page = db.get_page(slug, username).unwrap();
    if page.published == true {
        Ok(Json(page))
    } else {
        Err(Status::Unauthorized)
    }
}

#[put("/update-page/<id>", data = "<new_page>")]
pub fn update_page(
    auth: Credential<OAuth>,
    db: &State<MongoRepo>,
    id: &str,
    new_page: Json<PageUpdateRequest>,
) -> Result<Json<Page>, Status> {
    // get user from jwt token
    let token = &auth.token;

    let new_page = PageUpdateRequest {
        title: new_page.title.clone(),
        content: new_page.content.clone(),
        slug: new_page.slug.clone(),
        published: new_page.published.clone(),
    };

    let page: Page = db.update_page(id, new_page, token).unwrap();
    Ok(Json(page))
}

#[delete("/delete-page/<id>")]
pub fn delete_page(
    auth: Credential<OAuth>,
    db: &State<MongoRepo>,
    id: &str,
) -> Result<Json<DeleteResult>, Status> {
    // get user from jwt token
    let token = &auth.token;

    let page: DeleteResult = db.delete_page(id, token).unwrap();
    Ok(Json(page))
}
