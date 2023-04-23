use sea_orm::{ActiveModelTrait, DbConn, Set};

use crate::{
    entities::users,
    errors::ApiError,
    models::user::{User, UserCreate, UserUpdate},
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

pub async fn update_user_by_id(
    db: &DbConn,
    user_id: i32,
    update: UserUpdate,
) -> Result<(), ApiError> {
    let mut user: users::ActiveModel = match repositories::users::get_by_id(user_id, db).await? {
        Some(u) => u.into(),
        None => return Err(ApiError::unauthorized()),
    };

    match update.name {
        Some(name) => user.name = Set(name),
        None => (),
    }

    match update.username {
        Some(username) => {
            if repositories::users::get_by_username(&username, db)
                .await?
                .is_some()
            {
                return Err(ApiError::conflict());
            }
            user.username = Set(username);
        }
        None => (),
    }

    match update.password {
        Some(password) => user.password = Set(encryption::hash(password).await?),
        None => (),
    }

    _ = user.update(db).await?;

    Ok(())
}
