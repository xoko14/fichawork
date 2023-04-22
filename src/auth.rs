use axum::{
    async_trait,
    extract::FromRequest,
    headers::{authorization::Bearer, Authorization},
    http::Request,
    TypedHeader,
};

use crate::{errors::ApiError, utils::jwt};

pub struct UserAuth(pub i32);

#[async_trait]
impl<S, B> FromRequest<S, B> for UserAuth
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req, state)
                .await
                .map_err(|_| ApiError::unauthorized())?;
        let claims = jwt::verify_jwt(bearer.token())?;
        Ok(Self(claims.sub))
    }
}
