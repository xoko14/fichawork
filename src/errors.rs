use std::{error::Error, fmt::Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::BcryptError;
use sea_orm::DbErr;
use serde::Serialize;
use utoipa::ToSchema;

pub type ApiError = ProblemDetails;

#[derive(Serialize, Debug, ToSchema)]
pub struct ProblemDetails {
    title: String,
    status: u16,
    detail: String,
}

impl ProblemDetails {
    pub fn new(status_code: u16, title: &str, detail: &str) -> Self {
        Self {
            title: title.to_owned(),
            status: status_code,
            detail: detail.to_owned(),
        }
    }

    pub fn entity_not_found() -> Self {
        Self::new(404, "Not found", "Entity not found.")
    }

    pub fn db_conn_err(reason: &str) -> Self {
        Self::new(500, "Unable to connect to database", reason)
    }

    pub fn conflict() -> Self {
        Self::new(
            409,
            "Conflict",
            "Request resulted in a conflict with current server state.",
        )
    }

    pub fn default() -> Self {
        Self::new(
            500,
            "Internal server error",
            "An error was found while processing the request.",
        )
    }

    pub fn unauthorized() -> Self {
        Self::new(401, "Unauthorized", "Unauthorized")
    }
}

impl Display for ProblemDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ERROR {}] {}. {}", self.status, self.title, self.detail)
    }
}

impl Error for ProblemDetails {}

impl From<DbErr> for ProblemDetails {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::ConnectionAcquire => Self::db_conn_err("Connection pool was fully utilized."),
            DbErr::TryIntoErr {
                from: _,
                into: _,
                source: _,
            } => Self::default(),
            DbErr::Conn(_) => Self::db_conn_err("Problem with database connection"),
            DbErr::Exec(_) => Self::default(),
            DbErr::Query(_) => Self::default(),
            DbErr::ConvertFromU64(_) => Self::default(),
            DbErr::UnpackInsertId => Self::default(),
            DbErr::UpdateGetPrimaryKey => Self::conflict(),
            DbErr::RecordNotFound(_) => Self::entity_not_found(),
            DbErr::AttrNotSet(_) => Self::default(),
            DbErr::Custom(_) => Self::default(),
            DbErr::Type(_) => Self::default(),
            DbErr::Json(_) => Self::default(),
            DbErr::Migration(_) => Self::default(),
            DbErr::RecordNotInserted => Self::conflict(),
            DbErr::RecordNotUpdated => Self::conflict(),
        }
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for ProblemDetails {
    fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::default()
    }
}

impl From<BcryptError> for ProblemDetails {
    fn from(_: BcryptError) -> Self {
        Self::default()
    }
}

impl From<jsonwebtoken::errors::Error> for ProblemDetails {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        Self::unauthorized()
    }
}

impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}
