use std::env;

use anyhow::Error;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use dioxus::logger::tracing;
use tiberius::{AuthMethod, Config};

// endregion: --- Modules

pub type Db = Pool<ConnectionManager>;

fn get_env(name: &'static str) -> Result<String, Error> {
    let env = env::var(name)?;
    Ok(env)
}

fn build_config_db() -> Config {
    let mut config_db = Config::new();

    let db_host = get_env("MSSQL_HOST").unwrap_or("localhost".to_string());
    let db_port = get_env("MSSQL_PORT")
        .unwrap_or("1433".to_string())
        .parse::<u16>()
        .unwrap_or(1433);
    let db_name = get_env("MSSQL_DBNAME").unwrap_or("template".to_string());
    let db_username = get_env("MSSQL_USERNAME").unwrap_or("template_user".to_string());
    let db_password = get_env("MSSQL_PASSWORD").unwrap_or("template_pass".to_string());

    config_db.host(db_host);
    config_db.port(db_port);
    config_db.database(db_name);
    config_db.authentication(AuthMethod::sql_server(db_username, db_password));

    config_db.trust_cert(); // on production, it is not a good idea to do this

    config_db
}

pub async fn new_db_pool() -> Result<Db, Error> {
    let config = build_config_db();
    let mgr = bb8_tiberius::ConnectionManager::new(config);
    let pool = bb8::Pool::builder().max_size(5).build(mgr).await?;
    tracing::info!(
        "{:<12} - new_db_pool - {}",
        "STARTUP",
        "Creating Sql Server Connection Pools."
    );

    Ok(pool)
}
