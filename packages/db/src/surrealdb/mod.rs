use std::env;
use std::sync::LazyLock;

use anyhow::Error;
use dioxus::{logger::tracing, prelude::*};
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Database,
};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

fn get_env(name: &'static str) -> Result<String, Error> {
    let env = env::var(name)?;
    Ok(env)
}

pub async fn connection_db() -> Result<(), Error> {
    let db_host = get_env("SURREAL_HOST").unwrap_or("localhost".to_string());
    let db_port = get_env("SURREAL_PORT").unwrap_or("8000".to_string());
    let db_ns = get_env("SURREAL_NAMESPACE").unwrap_or("template".to_string());
    let db_name = get_env("SURREAL_DBNAME").unwrap_or("template".to_string());
    let db_username = get_env("SURREAL_USERNAME").unwrap_or("template_user".to_string());
    let db_password = get_env("SURREAL_PASSWORD").unwrap_or("template_pass".to_string());

    let address = format!("{db_host}:{db_port}");
    // Connect to the database
    DB.connect::<Ws>(address).await?;

    DB.signin(Database {
        namespace: &db_ns,
        username: &db_username,
        password: &db_password,
        database: &db_name,
    })
    .await?;

    let sql = "RETURN 'OK'";
    let mut response = DB.query(sql).await.unwrap().check().unwrap();

    let info_root: Option<String> = response.take(0).unwrap();

    tracing::info!(
        "{:<12} - connection_db {}: {:?}",
        "STARTUP",
        "testing query, result is:",
        info_root
    );

    Ok(())
}
