use crate::config::auth::{hash_password, verify_password};
use crate::models::user_model::{User, UserFromMongo};
use dotenv::dotenv;
use mongodb::bson::extjson::de::Error;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};
use std::env;

pub struct MongoRepo {
    col: Collection<UserFromMongo>,
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
        let col: Collection<UserFromMongo> = db.collection("user");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        //TODO: hash password

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
    pub fn get_user(&self, username: &str) -> Result<UserFromMongo, Error> {
        let filter = doc! {"username": username};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error Finding User");
        Ok(user_detail.expect("User not found"))
    }
    pub fn login(&self, username: &str, password: &str) -> Result<UserFromMongo, Error> {
        let filter = doc! {"username": username, "password": password};

        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error Finding User");

        // check password
        if user_detail.is_none() {
            //TODO: return error
        } else {
            let hashed_password = user_detail.as_ref().unwrap().password.clone();
            let is_valid = verify_password(password, hashed_password.as_str());
            if !is_valid {
                //TODO: return error
            }
        }

        Ok(user_detail.expect("User not found"))
    }
}
