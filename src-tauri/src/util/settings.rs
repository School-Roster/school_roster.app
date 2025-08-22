use crate::db::AppState;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tauri::api::dialog::FileDialogBuilder;
use tauri::api::path::app_data_dir;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    days: Vec<String>,
    #[serde(rename = "modulesPerDay")]
    pub modules_per_day: u32,
    #[serde(rename = "moduleDuration")]
    pub module_duration: u32,
    #[serde(rename = "durationUnit")]
    pub duration_unit: String,
    #[serde(rename = "hasBreaks")]
    pub has_breaks: bool,
    #[serde(rename = "breakCount")]
    pub break_count: u32,
    #[serde(rename = "breakDuration")]
    pub break_duration: u32,
    #[serde(rename = "breakPositions")]
    pub break_positions: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchoolInfo {
    pub name: String,
    pub logo_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            days: vec![
                "Lunes".to_string(),
                "Martes".to_string(),
                "Miércoles".to_string(),
                "Jueves".to_string(),
                "Viernes".to_string(),
            ],
            modules_per_day: 9,
            module_duration: 50,
            duration_unit: "minutes".to_string(),
            has_breaks: false,
            break_count: 1,
            break_duration: 30,
            break_positions: vec![2],
        }
    }
}

#[tauri::command]
pub async fn get_config(pool: tauri::State<'_, AppState>) -> Result<Config, String> {
    let cfg = sqlx::query(
        "
        SELECT value FROM config
        WHERE key = 'schedule_config'
        ",
    )
    .fetch_optional(&pool.db)
    .await
    .map_err(|e| format!("Error consiguiendo configuracion del horario: {}", e))?;

    match cfg {
        Some(row) => {
            let json_str: String = row.get("value");
            serde_json::from_str(&json_str)
                .map_err(|e| format!("No se pudo analizar la configuracion: {}", e))
        }
        None => Ok(Config::default()),
    }
}

#[tauri::command]
pub async fn save_config(
    pool: tauri::State<'_, AppState>,
    config: Config,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let cfg_json = serde_json::to_string(&config)
        .map_err(|e| format!("Failed to serialize config file: {}", e))?;

    sqlx::query(
        "
         INSERT INTO config (key, value)
         VALUES ('schedule_config', ?1)
         ON CONFLICT(key) DO UPDATE SET value = ?1",
    )
    .bind(&cfg_json)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed saving configuration: {}", e))?;

    app.emit_all("config_updated", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(())
}

/// Funcion para conseguir la informacion registrada de la escuela
#[tauri::command]
pub async fn get_school_info(pool: tauri::State<'_, AppState>) -> Result<SchoolInfo, String> {
    let school = sqlx::query("SELECT name, logo_path FROM school WHERE id = 1")
        .fetch_one(&pool.db)
        .await
        .map_err(|e| format!("Error getting school info: {}", e))?;

    Ok(SchoolInfo {
        name: school.get("name"),
        logo_path: school.get("logo_path"),
    })
}

/// Funcion para guardar (sobrescribir) nueva informacion de la escuela
#[tauri::command]
pub async fn save_school_info(
    pool: tauri::State<'_, AppState>,
    name: String,
    logo_path: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    println!("Logo path: {}", logo_path);
    sqlx::query("UPDATE school SET name = ?1, logo_path = ?2 WHERE id = 1")
        .bind(name)
        .bind(logo_path)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed saving school info: {}", e))?;

    app.emit_all("school_info_updated", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(())
}

#[tauri::command]
#[allow(unused)]
pub async fn select_school_logo(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let image_extensions = ["png", "jpg", "jpeg", "bmp", "webp", "svg"];

    let (tx, rx) = tokio::sync::oneshot::channel();

    FileDialogBuilder::new()
        .add_filter("Imágenes", &image_extensions)
        .set_title("Seleccionar logo de la escuela")
        .pick_file(move |path_option| {
            let _ = tx.send(path_option);
        });

    match rx.await {
        Ok(Some(path)) if path.exists() => {
            // Validate file size (max 5MB)
            let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
            if metadata.len() > 5 * 1024 * 1024 {
                return Err("El archivo es demasiado grande (máximo 5MB)".to_string());
            }

            // Get app data directory
            let app_data_dir = app_data_dir(&app.config())
                .ok_or("No se pudo obtener el directorio de datos de la aplicación")?;

            // Create resources directory if it doesn't exist
            let logo_dir = app_data_dir.join("school_logos");
            fs::create_dir_all(&logo_dir).map_err(|e| e.to_string())?;

            let filename = format!(
                "{}",
                path.file_name()
                    .ok_or("Nombre de archivo inválido")?
                    .to_string_lossy()
            );

            let dest_path = logo_dir.join(&filename);

            // Copy the file
            fs::copy(&path, &dest_path).map_err(|e| e.to_string())?;

            // Clean up old logos (keep last 5)
            let entries = fs::read_dir(&logo_dir).map_err(|e| e.to_string())?;
            let mut logos: Vec<_> = entries.filter_map(|e| e.ok()).collect();

            logos.sort_by(|a, b| {
                b.metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .cmp(&a.metadata().unwrap().modified().unwrap())
            });

            for old_logo in logos.into_iter().skip(5) {
                let _ = fs::remove_file(old_logo.path());
            }

            // Return full path
            Ok(Some(dest_path.to_string_lossy().into_owned()))
        }
        Ok(None) => Ok(None),
        Err(_) => Err("Error al abrir el diálogo".to_string()),
        _ => Err("Archivo no existe".to_string()),
    }
}
