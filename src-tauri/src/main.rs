// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::class::subjects::{
    get_subjects,
    create_subject,
    delete_subject,
    update_subject
};
use crate::class::teachers::{
    add_teacher,
    get_all_teachers,
    attach_subject,
};
use crate::db::{AppState, connect};
use tauri::Manager as _; // Necesario para poder usar manage()

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
            // Teachers
            add_teacher,
            get_all_teachers,
            attach_subject,
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let pool = connect(&app).await;
    app.manage(AppState { db: pool });
    // app.handle().plugin(tauri_plugin_store::Builder::default().build());
    app.run(|_, _| {});
}
