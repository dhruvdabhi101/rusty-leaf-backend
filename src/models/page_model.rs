use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub _id: ObjectId,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
    pub user_id: ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageCreateRequest {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageCreateResponse {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
    pub user_id: ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageGetResponse {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageUpdateRequest {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageUpdateResponse {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
}

#[derive(Deserialize)]
pub struct DeletePage {
    pub _id: ObjectId,
    pub user_id: ObjectId,
}
