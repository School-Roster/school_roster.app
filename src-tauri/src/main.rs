// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process;
use tauri::Manager as _; // Necesario para poder usar manage()

use crate::class::subjects::{
    get_subjects,
    create_subject,
    delete_subject,
    update_subject,
    get_subjects_with_teachers
};
use crate::class::teachers::{
    add_teacher,
    edit_teacher,
    get_all_teachers,
    delete_teacher
};
use crate::db::{AppState, connect};

mod db;
mod class;

#[tokio::main]
async fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Subjects
            create_subject,
            delete_subject,
            update_subject,
            get_subjects,
            get_subjects_with_teachers,
            // Teachers
            add_teacher,
            edit_teacher,
            get_all_teachers,
            delete_teacher
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    // Conectar a la base de datos y manejar posibles errores
    let pool = match connect(&app).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Database connection error: {}", err);
            // If the database exists but migrations are mismatched, give a more helpful message
            if err.to_string().contains("VersionMissing") || err.to_string().contains("VersionMismatch") {
                eprintln!("Migration version mismatch detected. Try one of the following:");
                eprintln!("1. Delete the existing database file and restart the application");
                eprintln!("2. Ensure your migration files in ./migrations/ are properly versioned");
            }
            process::exit(1);
        }
    };
    app.manage(AppState { db: pool });
    app.run(|_, _| {});
}
