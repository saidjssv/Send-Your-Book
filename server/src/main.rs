use dotenvy::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

// Función principal asincrónica
#[tokio::main]
async fn main() -> Result <(), sqlx::Error> {
    // Carga las variables de entorno desde el archivo .env
    dotenv().ok();

    // Obtiene la clave de conexión a la base de datos desde las variables de entorno
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    /* Crea un pool de conexiones a la base de datos PostgreSQL en Neon
        Configura un número máximo de conexiones de 5
        Establece la conexíon usando la url de la variable de entorno
        Espera la función asincronica para completar la conexión a la base de datos
    */
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Verifica si la conexión a la base de datos fue exitosa

    if sqlx::query("SELECT 1").execute(&pool).await.is_ok() {
        println!("Conexión a la base de datos realizada con éxito!");
    } else {
        println!("Error al conectar a la base de datos.");
    }
    Ok(())
}
