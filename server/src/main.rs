use dotenvy::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result <(), sqlx::Error> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    if sqlx::query("SELECT 1").execute(&pool).await.is_ok() {
        println!("Conexión a la base de datos realizada con éxito!");
    } else {
        println!("Error al conectar a la base de datos.");
    }
    Ok(())
}
