use anyhow::Error;
use db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, test_connection!");
    println!("tring to connect surrealdb");
    db::surrealdb::connection_db().await.unwrap();
    println!("Success Connect to SurraelDB");
    println!("tring to connect sql server");
    db::sqlserver::new_db_pool().await.unwrap();
    println!("Success Connect to SqlServer");
    Ok(())
}
