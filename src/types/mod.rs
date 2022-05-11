use serde::{Deserialize, Serialize}; 
use std::time::SystemTime;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: i32,
    pub data: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageUpload {
    pub caption: String,
    pub filename: String,
    pub isvenmo: bool,
    pub userid: i32,
    pub post_date: std::time::SystemTime,
}

impl Default for ImageUpload {
    fn default () -> ImageUpload {
        ImageUpload{userid: -1, caption: "".to_string(), filename: "".to_string(), 
                        isvenmo: false, post_date:SystemTime::now()}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSignup {
    pub email: String,
    pub password: String,
}

pub struct DBUser {
    pub email: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub userid: i32,
    pub exp: usize
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct APIResponse<P> {
    pub result: &'static str,
    pub message: P,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: String,
    pub image: String,
    pub caption: String,
    pub venmo: bool,
    pub upvotes: i32,
    pub downvotes: i32
}