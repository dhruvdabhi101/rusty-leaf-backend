use crate::models::user_model::User;
use dotenv::dotenv;
use mongodb::bson::extjson::de::Error;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};
use std::env;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(uri) => uri,
            Err(_) => panic!("MONGO_URI not found in .env file"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rusty-leaf");
        let col: Collection<User> = db.collection("user");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            username: new_user.username,
            password: new_user.password,
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
    pub fn get_user(&self, username: &str) -> Result<User, Error> {
        let filter = doc! {"username": username};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error Finding User");
        Ok(user_detail.expect("User not found"))
    }
    pub fn login(&self, username: &str, password: &str) -> Result<User, Error> {
        let filter = doc! {"username": username, "password": password};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error Finding User");
        Ok(user_detail.expect("User not found"))
    }
}
