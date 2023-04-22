use axum::{extract::State, Form, Json};
use sea_orm::DbConn;

use crate::{
    auth::UserAuth,
    errors::ApiError,
    models::{
        auth::{LoginCredentials, TokenPayload},
        user::{User, UserCreate, UserUpdate},
    },
    services,
};

#[utoipa::path(
    post,
    path = "/api/auth/token",
    request_body = LoginCredentials,
    responses(
        (status = 200, description = "Token claimed successfuly", body = TokenPayload),
        (status = 401, description = "Unable to authenticate", body = ProblemDetails)
    )
)]
pub async fn get_token(
    State(db): State<DbConn>,
    Form(credentials): Form<LoginCredentials>,
) -> Result<Json<TokenPayload>, ApiError> {
    let result = services::auth::auth(&db, &credentials).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "Success", body = User),
        (status = 401, description = "Unable to authenticate", body = ProblemDetails)
    ),
    security(
        ("Bearer token"=["user"])
    )
)]
pub async fn get_logged_user(
    State(db): State<DbConn>,
    UserAuth(user_id): UserAuth,
) -> Result<Json<User>, ApiError> {
    let result = services::users::get_user_by_id(&db, user_id).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = UserCreate,
    responses(
        (status = 200, description = "User created successfuly")
    )
)]
pub async fn create_user(
    State(db): State<DbConn>,
    Json(user): Json<UserCreate>,
) -> Result<(), ApiError> {
    services::users::create_user(&db, user).await
}

pub async fn update_logged_user(
    State(db): State<DbConn>,
    Json(update): Json<UserUpdate>,
) -> Result<(), ApiError> {
    todo!()
}
