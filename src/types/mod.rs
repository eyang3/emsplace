use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Person {
    id: i32,
    data: String,
}