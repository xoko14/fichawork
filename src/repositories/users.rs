use crate::{
    entities::{prelude::Users, users},
    errors::ApiError,
    models::user::UserCreate,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};

pub type UserModel = users::Model;

pub async fn get_by_id(id: i32, db: &DbConn) -> Result<Option<UserModel>, ApiError> {
    let user = Users::find_by_id(id).one(db).await?;
    Ok(user)
}

pub async fn get_by_username(username: &str, db: &DbConn) -> Result<Option<UserModel>, ApiError> {
    let user = Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await?;
    Ok(user)
}

pub async fn create_user(
    db: &DbConn,
    user: UserCreate,
    hashed_pwd: String,
) -> Result<(), ApiError> {
    let user = users::ActiveModel {
        username: Set(user.username),
        password: Set(hashed_pwd),
        name: Set(user.name),
        ..Default::default()
    };
    _ = user.insert(db).await?;
    Ok(())
}
