use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::query("SELECT 1").execute(&pool).await?;

    println!("Conexión a TiDB realizada con éxito!");

    Ok(())
}