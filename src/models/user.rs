use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::users;

#[derive(Serialize, Debug, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UserCreate {
    pub username: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

impl From<users::Model> for User {
    fn from(dbmodel: users::Model) -> Self {
        Self {
            id: dbmodel.id,
            username: dbmodel.username,
            name: dbmodel.name,
        }
    }
}
