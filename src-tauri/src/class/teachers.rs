use futures::TryStreamExt;
use crate::db::AppState;
use sqlx::prelude::FromRow;
use serde::{Deserialize, Serialize};
use crate::class::subjects::Subject;

/// Estructura de un profesor
/// Se utiliza para mapear los datos de un profesor de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Teacher {
    pub id: i16,
    pub name: String,
    pub father_lastname: String,
    pub mother_lastname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub degree: Option<String>,
    pub commisioned_hours: Option<i16>, // Total de horas
    pub active_hours: Option<i16>, // Horas activas en el programa
    pub performance: Option<i16>, // Desempeño
    pub subjects: Option<Vec<Subject>>, // Materias que imparte (option porque en algun momento podria no tener materias)
}

/// Funcion para agregar una materia a un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `teacher_id` - ID del profesor
/// * `subject_id` - ID de la materia
/// Retorna un resultado vacio si la operacion fue exitosa
/// Desde la interfaz de usuario, se selecciona un profesor y una materia,
/// y se llama a esta funcion para agregar la materia al profesor
#[tauri::command]
pub async fn attach_subject(
    pool: tauri::State<'_, AppState>,
    teacher_id: i16,
    subject_id: i16,
) -> Result<(), String> {
    sqlx::query("INSERT INTO teacher_subject (teacher_id, subject_id) VALUES (?1, ?2)")
        .bind(teacher_id)
        .bind(subject_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to attach subject to teacher: {}", e))?;

    Ok(())
}

/// Funcion para frontend para obtener materias para un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `teacher_id` - ID del profesor
/// Retorna un vector con las materias que imparte un profesor
/// Se llama desde la interfaz de usuario para obtener las materias que imparte un profesor
#[tauri::command]
pub async fn get_subjects_for_teacher(
    pool: tauri::State<'_, AppState>,
    teacher_id: i16
) -> Result<Vec<Subject>, String> {
    let subjects: Vec<Subject> = sqlx::query_as::<_, Subject>("
        SELECT * FROM subjects
        WHERE id IN (
            SELECT subject_id FROM teacher_subject
            WHERE teacher_id = ?1
        )
    ")
        .bind(teacher_id)
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    Ok(subjects)
}

/// Funcion para remover una materia de un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `teacher_id` - ID del profesor
/// * `subject_id` - ID de la materia
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para remover una materia de un profesor
#[tauri::command]
pub async fn detach_subject(
    pool: tauri::State<'_, AppState>,
    teacher_id: i16,
    subject_id: i16,
) -> Result<(), String> {
    sqlx::query("DELETE FROM teacher_subject WHERE teacher_id = ?1 AND subject_id = ?2")
        .bind(teacher_id)
        .bind(subject_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to detach subject from teacher: {}", e))?;

    Ok(())
}

// TODO: Funcion para actualizar.
