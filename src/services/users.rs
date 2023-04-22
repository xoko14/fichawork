use sea_orm::DbConn;

use crate::{
    errors::ApiError,
    models::user::{User, UserCreate},
    repositories,
    utils::encryption,
};

pub async fn create_user(db: &DbConn, user: UserCreate) -> Result<(), ApiError> {
    if repositories::users::get_by_username(&user.username, db)
        .await?
        .is_some()
    {
        return Err(ApiError::conflict());
    }

    let hashed_pwd = encryption::hash(user.password.clone()).await?;

    repositories::users::create_user(db, user, hashed_pwd).await?;

    Ok(())
}

pub async fn get_user_by_id(db: &DbConn, user_id: i32) -> Result<User, ApiError> {
    let db_user = match repositories::users::get_by_id(user_id, db).await? {
        Some(u) => u,
        None => return Err(ApiError::entity_not_found()),
    };

    Ok(User::from(db_user))
}
