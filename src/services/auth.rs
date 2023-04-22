use sea_orm::DbConn;

use crate::{
    errors::ApiError,
    models::auth::{LoginCredentials, TokenPayload},
    repositories,
    utils::{encryption, jwt::Claims},
};

pub async fn auth(db: &DbConn, credentials: &LoginCredentials) -> Result<TokenPayload, ApiError> {
    if credentials.grant_type != "password" {
        return Err(ApiError::unauthorized());
    }

    match credentials.scope.as_str() {
        "user" => auth_user(db, credentials).await,
        _ => return Err(ApiError::unauthorized()),
    }
}

async fn auth_user(db: &DbConn, credentials: &LoginCredentials) -> Result<TokenPayload, ApiError> {
    let user = match repositories::users::get_by_username(&credentials.username, db).await? {
        Some(u) => u,
        None => return Err(ApiError::unauthorized()),
    };

    if !encryption::verify_hash(credentials.password.clone(), user.password).await? {
        return Err(ApiError::unauthorized());
    }

    let token = TokenPayload {
        access_token: Claims::new(user.id, "user").sign()?,
        token_type: "Bearer".to_owned(),
    };
    Ok(token)
}
