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
    logo_path: Option<String>,
    app: tauri::AppHandle,
) -> Result<(), String> {
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
pub async fn select_school_logo(window: tauri::Window) -> Result<Option<String>, String> {
    // Definir las extensiones de imagen soportadas
    let image_extensions = ["png", "jpg", "jpeg", "bmp", "webp", "svg"];

    // Crear un canal para recibir el resultado del diálogo
    let (tx, rx) = tokio::sync::oneshot::channel();
    // let (tx, rx) = std::sync::mpsc::channel();

    // Abrir el diálogo de archivos
    FileDialogBuilder::new()
        .add_filter("Imágenes", &image_extensions)
        .set_title("Seleccionar logo de la escuela")
        .pick_file(move |path_option| {
            let _ = tx.send(path_option);
        });

    // Esperar el resultado del diálogo
    match rx.await {
        Ok(path_option) => {
            match path_option {
                Some(path) => {
                    // Validar que el archivo existe
                    if path.exists() {
                        Ok(Some(path.to_string_lossy().into_owned()))
                    } else {
                        Err("El archivo seleccionado no existe".to_string())
                    }
                }
                None => Ok(None), // Usuario canceló la selección
            }
        }
        Err(_) => Err("Error interno al abrir el diálogo de archivos".to_string()),
    }
}
