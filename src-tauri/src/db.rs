/*
    Conecta la base de datos con la aplicacion.
    sqlx es un crate que nos permite interactuar con la base de datos de forma asincrona.
    la base de datos sera en sqlite.
*/

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::fs;
use std::path::PathBuf;

use tauri::App;

pub const DB_NAME: &str = "school_roster.sqlite";

// Pool de la base de datos para interactuar con ella en la aplicacion.
pub type DbPool = Pool<Sqlite>;

pub struct AppState {
    pub db: DbPool,
}

/*
*** Funcion para conectar a la base de datos ***
    Retorna un pool es usado para interactuar con la base de datos.
*/
pub async fn connect(app: &App) -> Result<DbPool, Box<dyn std::error::Error>> {
    println!("Connecting to database...");

    let db_path = setup_db_path(app)?;

    println!("Database path: {}", db_path.display());

    create_database_file(&db_path)?;

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", db_path.to_string_lossy()))
        .await?;

    // Checar si se necesita correr migraciones
    match handle_migrations(&pool).await {
        Ok(_) => {
            println!("Database migrations completed successfully!");
            Ok(pool)
        }
        Err(e) => {
            if e.to_string().contains("VersionMissing") {
                println!("Attemting to reset migrations...");
                sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations")
                    .execute(&pool)
                    .await?;

                // Intentar de nuevo
                handle_migrations(&pool).await?; // TODO: Esto puede ser peligroso, hacer muchas pruebas para comprobar
                Ok(pool)
            } else {
                Err(e.into())
            }
        }
    }
}

fn setup_db_path(app: &App) -> Result<PathBuf, std::io::Error> {
    // Obtiene la ruta de la aplicacion.
    let mut path = app.path_resolver().app_data_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to get data directory")
    })?;

    fs::create_dir_all(&path)?;
    path.push(DB_NAME);
    Ok(path)
}

fn create_database_file(path: &PathBuf) -> Result<(), std::io::Error> {
    match fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
    {
        Ok(_) => println!("Database file created successfully"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => println!("Datbase already exists"),
            _ => return Err(e),
        },
    }

    Ok(())
}

async fn handle_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
