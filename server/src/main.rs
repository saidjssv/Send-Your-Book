use dotenvy::dotenv;
use sqlx::mysql::{MySqlPoolOptions, MySqlConnectOptions, MySqlSslMode};
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    // Carga las variables de entorno desde el archivo .env
    dotenv().ok();

    // Define la URL de conexión a la base de datos desde la variable de entorno
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Configura las opciones de conexión a la base de datos: Require el modo SSL para que las conexiones sean seguras 
    // y no existan errores de conexión
    let options = MySqlConnectOptions::from_str(&database_url)?
        .ssl_mode(MySqlSslMode::Required);

    // Crea un pool de conexiones a la base de datos con un máximo de 5 conexiones
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Ejecuta una consulta simple para verificar la conexión a la base de datos
    let testquery = sqlx::query("SELECT 1").execute(&pool).await?;

    println!("Conexión a TiDB");
    println!("Conexión a TiDB realizada con éxito!");

    Ok(())
}