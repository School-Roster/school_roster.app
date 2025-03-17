use crate::db::AppState;
use futures::TryStreamExt; // Para poder usar try_next() en los streams
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Row};

/// Estructura de la asignacion
/// Se utiliza para mapear los datos que van y vienen de la base de datos
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Assignment {
    pub id: Option<i16>,
    pub group_id: i16,
    pub day: String,
    pub module_index: i16,
    pub subject_id: i16,
    pub teacher_id: i16,
    pub classroom_id: i16,
    pub subject_shorten: String,
    pub subject_color: String,
}

/// Funcion que asigna una materia al modulo
#[allow(dead_code, unused)]
// #[tauri::command]
#[tauri::command(rename_all = "snake_case")]
pub async fn save_assignment(
    pool: tauri::State<'_, AppState>,
    group_id: i32,
    day: &str,
    module_index: i32,
    subject_id: i32,
    teacher_id: i32,
) -> Result<(), String> {
    sqlx::query(
        "
        INSERT INTO assignments (group_id, day, module_index, subject_id, teacher_id)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ON CONFLICT (group_id, day, module_index) DO UPDATE
        SET subject_id = excluded.subject_id, teacher_id = excluded.teacher_id
        ",
    )
    .bind(group_id)
    .bind(day)
    .bind(module_index)
    .bind(subject_id)
    .bind(teacher_id)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Error creating the assignment: {}", e))?;

    Ok(())
}

/// Funcion para obtener una materia al modulo
/// Retorna la asignacion y el
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_assignment(
    pool: tauri::State<'_, AppState>,
    group_id: i32,
    day: &str,
    module_index: i32,
) -> Result<Option<(i32, i32)>, String> {
    let assignment = sqlx::query(
        "
        SELECT subject_id, teacher_id
        FROM assignments
        WHERE group_id = ?1 AND day = ?2 AND module_index = ?3
        ",
    )
    .bind(group_id)
    .bind(day)
    .bind(module_index)
    .fetch_optional(&pool.db)
    .await
    .map_err(|e| format!("Error getting assignments {}", e))?;

    let result: Option<(i32, i32)> = assignment.map(|row| {
        let sid: i32 = row.get("subject_id");
        let tid: i32 = row.get("teacher_id");
        (sid, tid)
    });

    Ok(result)
}

/// Funcion que retorna todas las asignaciones hechas
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_all_assignments(
    pool: tauri::State<'_, AppState>,
) -> Result<Vec<Assignment>, String> {
    let result: Vec<Assignment> = sqlx::query_as::<_, Assignment>(
        r#"
        SELECT a.id, a.group_id, a.day, a.module_index, a.teacher_id, 
               s.id as subject_id, s.name as subject_name, s.color as subject_color, s.shorten as subject_shorten
        FROM assignments a
        JOIN subjects s ON a.subject_id = s.id
        "#,
    )
    .fetch(&pool.db)
    .try_collect()
    .await
    .map_err(|e| format!("An error occurred while getting the assignments: {}", e))?;

    Ok(result)
}

/// Funcion para eliminar una asignacion
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn delete_assignment(
    pool: tauri::State<'_, AppState>,
    assign_id: i32,
) -> Result<(), String> {
    sqlx::query("DELETE FROM assignments WHERE id=?")
        .bind(assign_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error deleting assignment: {}", e))?;

    Ok(())
}
