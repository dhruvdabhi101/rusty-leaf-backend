use crate::config::auth::{hash_password, verify_password};
use crate::config::jwt::decode_jwt;
use crate::models::page_model::{Page, PageCreateRequest, PageCreateResponse, PageUpdateRequest};
use crate::models::user_model::{User, UserFromMongo};
use darkdown::converter::converter::Converter;
use dotenv::dotenv;
use mongodb::bson::extjson::de::Error;
use mongodb::results::DeleteResult;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};
use std::env;
use std::str::FromStr;

use super::error::{PageError, UserError};

pub struct MongoRepo {
    col: Collection<UserFromMongo>,
    page: Collection<Page>,
}

impl MongoRepo {
    pub fn init(mongouri: String) -> Self {
        let uri = mongouri;
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rusty-leaf");
        let col: Collection<UserFromMongo> = db.collection("user");
        let page: Collection<Page> = db.collection("page");
        MongoRepo { col, page }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let hashed_password: String = hash_password(new_user.password);

        let new_doc = UserFromMongo {
            _id: ObjectId::new(),
            username: new_user.username,
            password: hashed_password,
            email: new_user.email,
            name: new_user.name,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error Creating User");
        Ok(user)
    }
    pub fn get_user(&self, id: &str) -> Result<UserFromMongo, Error> {
        let user_id = bson::oid::ObjectId::from_str(id);
        let filter = doc! {"_id": user_id.unwrap()};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error Finding User");
        Ok(user_detail.expect("User not found"))
    }

    pub fn update_user(&self, new_user: User, token: &str) -> Result<UserFromMongo, UserError> {
        let user_id = decode_jwt(token).unwrap().sub;

        let user_id = ObjectId::from_str(user_id.as_str()).unwrap();

        // get user and see if user_id matches
        let filter = doc! {"_id": user_id};
        let user = self
            .col
            .find_one(filter.clone(), None)
            .ok()
            .expect("Error Finding User");

        if user.is_none() {
            return Err(UserError::NotFound);
        } else {
            let user: UserFromMongo = user.unwrap();
            if user._id != user_id {
                return Err(UserError::InvalidCredentials);
            }
        }

        let hashed_password: String = hash_password(new_user.password);

        let filter = doc! {"username": new_user.username.clone()};
        let update = doc! {"$set": {"username": new_user.username, "password": hashed_password, "email": new_user.email, "name": new_user.name}};
        let user = self
            .col
            .find_one_and_update(filter, update, None)
            .ok()
            .expect("Error Updating User");
        return Ok(user.unwrap());
    }

    pub fn login(&self, username: &str, password: &str) -> Result<UserFromMongo, UserError> {
        let filter = doc! {"username": username};

        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error Finding User");

        if user_detail.is_some() {
            let user: UserFromMongo = user_detail.unwrap();
            let is_valid = verify_password(password, user.password.as_str());
            if !is_valid {
                return Err(UserError::InvalidPassword);
            }
            return Ok(user);
        } else {
            return Err(UserError::NotFound);
        }
    }

    // Page CRUD
    pub fn create_page(
        &self,
        new_page: PageCreateRequest,
        token: &String,
    ) -> Result<PageCreateResponse, Error> {
        // get user id from jwt token
        let user_id = decode_jwt(token.as_str()).unwrap().sub;
        // let user_obj: ObjectId = ;

        let user_id = ObjectId::from_str(user_id.as_str()).unwrap();
        // create page

        // convert content from darkdown to html

        let darkdown_content = new_page.content.clone();

        let page = Page {
            _id: ObjectId::new(),
            title: new_page.title.clone(),
            content: darkdown_content.clone(),
            published: new_page.published.clone(),
            slug: new_page.slug.clone(),
            user_id: user_id.clone(),
        };
        self.page
            .insert_one(page, None)
            .ok()
            .expect("Error Creating Page");

        let page_response = PageCreateResponse {
            title: new_page.title,
            content: darkdown_content.clone(),
            published: new_page.published,
            slug: new_page.slug,
            user_id,
        };
        Ok(page_response)
    }

    pub fn get_page(&self, slug: &str, username: &str) -> Result<Page, PageError> {
        let filer = doc! { "username": username };
        let user = self
            .col
            .find_one(filer, None)
            .ok()
            .expect("Error Finding User");

        if user.is_some() {
            let user: UserFromMongo = user.unwrap();
            let user_id = user._id;
            let filter = doc! {"slug": slug, "user_id": user_id};
            let page = self
                .page
                .find_one(filter, None)
                .ok()
                .expect("Error Finding Page");

            if page.is_some() {
                let mut page: Page = page.unwrap();
                page.content = Converter::new().convert_to_html(&page.content);
                return Ok(page);
            } else {
                return Err(PageError::NotFound);
            }
        } else {
            return Err(PageError::NotFound);
        }
    }

    pub fn update_page(
        &self,
        id: &str,
        new_page: PageUpdateRequest,
        token: &String,
    ) -> Result<Page, PageError> {
        #[warn(unused_variables)]
        let user_id = decode_jwt(token.as_str()).unwrap().sub;

        let id = ObjectId::from_str(id).unwrap();
        let filter = doc! {"_id": id};
        let update = doc! {"$set": {"title": new_page.title, "content": new_page.content, "published": new_page.published, "slug": new_page.slug}};
        let page = self
            .page
            .find_one_and_update(filter, update, None)
            .ok()
            .expect("Error Updating Page");
        return Ok(page.unwrap());
    }

    pub fn get_pages(&self, token: &str) -> Result<Vec<Page>, PageError> {
        let user_id = decode_jwt(token).unwrap().sub;
        let user_id = ObjectId::from_str(user_id.as_str()).unwrap();

        let filter = doc! {"user_id": user_id};
        let pages = self
            .page
            .find(filter, None)
            .ok()
            .expect("Error finding pages");

        let serial: Vec<Result<Page, mongodb::error::Error>> = pages.collect::<Vec<_>>();
        let mut results: Vec<Page> = Vec::new();
        for page in serial {
            results.push(page.unwrap());
        }

        return Ok(results);
    }

    pub fn get_page_by_id(&self, id: &str, token: &str) -> Result<Page, PageError> {
        let user_id = decode_jwt(token).unwrap().sub;
        let user_id = ObjectId::from_str(user_id.as_str()).unwrap();
        let page_id = ObjectId::from_str(id).unwrap();

        let filter = doc! { "_id": page_id, "user_id": user_id };
        let page = self
            .page
            .find_one(filter, None)
            .ok()
            .expect("Error finding page");

        if let Some(page) = page {
            Ok(page)
        } else {
            Err(PageError::NotFound)
        }
    }

    pub fn delete_page(&self, id: &str, token: &String) -> Result<DeleteResult, PageError> {
        let user_id = decode_jwt(token.as_str()).unwrap().sub;
        let user_id = ObjectId::from_str(user_id.as_str()).unwrap();

        let id = ObjectId::from_str(id).unwrap();
        let filter = doc! {"_id": id};

        // get page and see if user_id matches
        let page = self
            .page
            .find_one(filter.clone(), None)
            .ok()
            .expect("Error Finding Page");
        if page.is_none() {
            return Err(PageError::NotFound);
        } else {
            let page: Page = page.unwrap();
            if page.user_id != user_id {
                return Err(PageError::InternalError);
            }
        }

        let page = self
            .page
            .delete_one(filter, None)
            .ok()
            .expect("Error Deleting Page");
        Ok(page)
    }
}
