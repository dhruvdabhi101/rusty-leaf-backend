use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub username: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
}
