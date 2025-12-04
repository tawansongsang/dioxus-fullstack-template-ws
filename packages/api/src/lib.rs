//! This crate contains all shared fullstack server functions.
use anyhow::{anyhow, Error};
#[cfg(feature = "server")]
use db::surrealdb::DB;

use dioxus::logger::tracing;
use dioxus::prelude::*;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[get("/api/test_connection")]
pub async fn test_connection() -> Result<String, Error> {
    use db::sqlserver;

    let surql = "RETURN 'OK';";
    let mut res = DB.query(surql).await?;
    let test_conn_res: Option<String> = res.take(0)?;
    let test_conn_result = test_conn_res
        .ok_or("Data not Found from `REUTRN 'OK';`".to_owned())
        .map_err(|e| anyhow!(e))?;

    tracing::info!("{}", test_conn_result);

    let pool = sqlserver::get_db_pool().await?;
    let mut client = pool.get().await?;
    let stream = client.simple_query("SELECT @@version;").await?;
    let result = stream
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| {
            let val: &str = row.get(0).unwrap();
            String::from(val)
        })
        .collect::<Vec<_>>();

    info!(
        "{} - test_connection {} {:?}",
        "STARTUP", "testing query, result is", result
    );

    Ok("Success".to_string())
}
