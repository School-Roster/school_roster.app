use crate::db::AppState;
use serde::{Deserialize, Serialize};
use sqlx::Row;
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
