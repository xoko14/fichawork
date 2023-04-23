use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};

use crate::{errors::ApiError, utils::jwt};

pub struct UserAuth(pub i32);

#[async_trait]
impl<S> FromRequestParts<S> for UserAuth
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| ApiError::unauthorized())?;
        let claims = jwt::verify_jwt(bearer.token())?;
        Ok(Self(claims.sub))
    }
}
