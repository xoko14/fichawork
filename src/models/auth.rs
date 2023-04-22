use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginCredentials {
    pub grant_type: String,
    pub scope: String,
    pub username: String,
    pub password: String,
}
