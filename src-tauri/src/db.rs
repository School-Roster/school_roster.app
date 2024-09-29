/* 
    Conecta la base de datos con la aplicacion.
    sqlx es un crate que nos permite interactuar con la base de datos de forma asincrona.
    la base de datos sera en sqlite.
*/

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use tauri::App;

const DB_NAME: &str = "school_roster.sqlite";

// Pool de la base de datos para interactuar con ella en la aplicacion.
pub type DbPool = Pool<Sqlite>;
pub struct AppState {
    pub db: DbPool,
}

/*
*** Funcion para conectar a la base de datos ***
    Retorna un pool es usado para interactuar con la base de datos.
*/
pub async fn connect(app: &App) -> DbPool {
    println!("Connecting to database...");
    // Obtiene la ruta de la aplicacion.
    let mut path = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get data directory");

    // Crea el directorio de datos si no existe.
    match std::fs::create_dir_all(&path) {
        Ok(_) => println!("Data directory created"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Failed to create data directory: {}", e),
        },
    }

    path.push(DB_NAME);

    let result = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        // .read(true)
        .open(&path);

    match result {
        Ok(_) => println!("Database created"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => println!("Database already exists"),
            _ => panic!("Failed to create database: {}", e),
        },
    }

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", path.to_string_lossy()))
        .await
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
