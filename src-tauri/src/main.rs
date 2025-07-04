// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod class;
mod db;
mod util;

use crate::db::{connect, AppState};
use std::process;
use tauri::Manager as _; // Necesario para poder usar manage()

#[tokio::main]
async fn main() {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Subjects
            crate::class::subjects::create_subject,
            crate::class::subjects::create_subjects,
            crate::class::subjects::delete_subject,
            crate::class::subjects::delete_subjects,
            crate::class::subjects::update_subject,
            crate::class::subjects::get_subjects,
            crate::class::subjects::get_subjects_with_teachers,
            // Teachers
            crate::class::teachers::add_teacher,
            crate::class::teachers::create_teachers,
            crate::class::teachers::edit_teacher,
            crate::class::teachers::get_all_teachers,
            crate::class::teachers::delete_teacher,
            crate::class::teachers::delete_teachers,
            crate::class::teachers::has_teachers,
            crate::class::teachers::check_teacher_availability,
            // Groups
            crate::class::groups::create_group,
            crate::class::groups::create_groups,
            crate::class::groups::update_group,
            crate::class::groups::delete_group,
            crate::class::groups::delete_groups,
            crate::class::groups::get_groups,
            crate::class::groups::create_students,
            crate::class::groups::get_students_by_group,
            crate::class::groups::save_excel_file,
            // Classrooms
            crate::class::classrooms::get_classrooms,
            crate::class::classrooms::create_classroom,
            crate::class::classrooms::create_classrooms,
            crate::class::classrooms::delete_classroom,
            crate::class::classrooms::delete_classrooms,
            crate::class::classrooms::update_classroom,
            crate::class::classrooms::remove_classroom_assignment,
            crate::class::classrooms::assign_classroom_to_assignment,
            crate::class::classrooms::check_classroom_availability,
            // Utils
            crate::util::assignments::get_all_assignments,
            crate::util::assignments::get_assignment,
            crate::util::assignments::save_assignment,
            crate::util::assignments::delete_assignment,
            crate::util::generate::generate_schedule,
            crate::util::settings::get_config,
            crate::util::settings::save_config,
            crate::util::settings::get_school_info,
            crate::util::settings::save_school_info,
            crate::util::settings::select_school_logo,
            crate::util::file_handler::export_file,
            crate::util::file_handler::import_file,
            crate::util::file_handler::delete_all_data,
            crate::util::file_handler::export_pdf_file,
            crate::util::file_handler::export_file,
            crate::util::file_handler::import_file,
            crate::util::file_handler::delete_all_data,
            crate::util::ai::init_model,
            crate::util::ai::query_ai,
            crate::util::ai::check_api_key
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let pool = match connect(&app).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Database connection error: {}", err);
            // If the database exists but migrations are mismatched, give a more helpful message
            if err.to_string().contains("VersionMissing")
                || err.to_string().contains("VersionMismatch")
            {
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
