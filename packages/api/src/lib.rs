//! This crate contains all shared fullstack server functions.

use dioxus::logger::tracing;
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod surreal_db {
    use db::surrealdb::{self, DB};

    pub async fn test() -> String {
        surrealdb::connection_db().await.unwrap();
        let surql = "RETURN 'OK'";
        let tmp = DB.query(surql).await.unwrap().check().unwrap();
        format!("{:?}", tmp)
    }
}

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[get("/api/test_connection")]
pub async fn test_connection() -> Result<String, ServerFnError> {
    let surql = "RETURN 'OK'";
    let tmp = surreal_db::test().await;
    tracing::info!("{}", tmp);
    Ok("Success".to_string())
}
