//! This crate contains all shared fullstack server functions.
#[cfg(feature = "server")]
use db::sqlserver;
#[cfg(feature = "server")]
use db::surrealdb::SURREAL_DB;

use dioxus::fullstack::{body::Body, response::Response, StatusCode};
use dioxus::logger::tracing;
use dioxus::{prelude::*, CapturedError};

#[get("/api/livez")]
pub async fn livez() -> Result<Response, ServerFnError> {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::OK;
    Ok(response)
}

#[get("/api/readyz")]
pub async fn readyz() -> Result<Response, CapturedError> {
    let surql = "RETURN 'OK';";
    let mut res = SURREAL_DB.query(surql).await?;
    let test_conn_res: Option<String> = res.take(0)?;
    let test_conn_result = test_conn_res
        .ok_or("Data not Found from `REUTRN 'OK';`".to_owned())
        .map_err(|e| CapturedError::msg(e))?;

    tracing::info!("{} - readyz - SURREALDB - {}", "HEALTH", test_conn_result);

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

    tracing::info!("{} - readyz - SQLSERVER - {:?}", "HEALTH", result);

    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::OK;
    Ok(response)
}
