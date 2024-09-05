use futures::TryStreamExt; // Para poder usar try_next() en los streams
use crate::db::AppState;
use sqlx::prelude::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Subject {
    pub id: i16,
    pub name: String,
    pub shorten: String,
    pub color: String,
    pub spec: String,
}

/* Funcion para crear una nueva materia */
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn create_subject(
    pool: tauri::State<'_, AppState>,
    name: String,
    shorten: String,
    color: String,
    spec: String,
) -> Result<(), String> {
    println!("Creating subject: {} {} {} {}", name, shorten, color, spec);
    sqlx::query("INSERT INTO subjects (name, shorten, color, spec) VALUES (?1, ?2, ?3, ?4)")
        .bind(name)
        .bind(shorten)
        .bind(color)
        .bind(spec)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to create subject: {}", e))?;

    println!("Subject created successfully");

    Ok(())
}

/* Funcion para obtener todas las materias */
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_subjects(pool: tauri::State<'_, AppState>) -> Result<Vec<Subject>, String> {
    let subjects: Vec<Subject> = sqlx::query_as::<_, Subject>("SELECT * FROM subjects")
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    Ok(subjects)
}

/* Funcion para eliminar una materia */
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_subject(pool: tauri::State<'_, AppState>, id: i16) -> Result<(), String> {
    sqlx::query("DELETE FROM subjects WHERE id = ?1")
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete subject: {}", e))?;

    Ok(())
}

/* Funcion para actualizar una materia */
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn update_subject(
    pool: tauri::State<'_, AppState>,
    id: i16,
    name: String,
    shorten: String,
    color: String,
    spec: String,
) -> Result<(), String> {
    sqlx::query("UPDATE subjects SET name = ?1, shorten = ?2, color = ?3, spec = ?4 WHERE id = ?5")
        .bind(name)
        .bind(shorten)
        .bind(color)
        .bind(spec)
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to update subject: {}", e))?;

    Ok(())
}
