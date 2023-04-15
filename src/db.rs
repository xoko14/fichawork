use sea_orm::{Database, DatabaseConnection};

pub async fn get_db_conn(conn_str: String) -> DatabaseConnection {
    Database::connect(&conn_str)
        .await
        .expect("Unable to connect to DB")
}
