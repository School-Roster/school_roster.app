use crate::db::AppState;
use std::{fs, path::PathBuf};
use tauri::{api::dialog, Manager, Window};

/// Function to export the file (.roster)
#[tauri::command]
pub async fn export_database(
    handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    window: Window,
) -> Result<String, String> {
    // Use a channel to communicate between the callback and async code
    let (tx, rx) = std::sync::mpsc::channel();

    // Show the save dialog
    dialog::FileDialogBuilder::new()
        .add_filter("Roster Files", &["roster"])
        .set_title("Guardar datos")
        .save_file(move |file_path| {
            tx.send(file_path).unwrap();
        });

    // Wait for the user's selection
    let file_path = match rx.recv().unwrap() {
        Some(path) => path,
        None => return Err("Cancelado por el usuario".to_string()),
    };

    // Ensure the extension is '.roster'
    let file_path = if file_path.extension().is_none() {
        file_path.with_extension("roster")
    } else {
        file_path
    };

    let app_data_dir = handle
        .path_resolver()
        .app_data_dir()
        .ok_or_else(|| "Could not resolve app data directory".to_string())?;

    let db_path = app_data_dir.join("school_roster.sqlite");

    // Close database connections to ensure all data is written
    drop(state);

    // Copy the database to the location
    match fs::copy(&db_path, &file_path) {
        Ok(_) => {
            window
                .emit("export-success", file_path.to_string_lossy().to_string())
                .map_err(|e| e.to_string())?;
            Ok(format!(
                "Database exported successfully to {}",
                file_path.display()
            ))
        }
        Err(e) => Err(format!("Failed to export database: {}", e)),
    }
}
