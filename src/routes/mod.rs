use axum::{extract::State, response::IntoResponse};
use sea_orm::DatabaseConnection;

pub async fn dbtest(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    "connected to db"
}
