use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct User {
    id: i32,
    username: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct UserCreate {
    username: String,
    name: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct UserUpdate {
    username: Option<String>,
    password: Option<String>,
    name: Option<String>,
}
