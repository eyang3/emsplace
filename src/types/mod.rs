use serde::{Deserialize, Serialize}; 

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: i32,
    pub data: String,
}