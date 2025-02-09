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
    pub teacher_id: i16,
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
    println!("{}", teacher_id);
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
    let result: Vec<Assignment> = sqlx::query_as::<_, Assignment>("SELECT * FROM assignments")
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| format!("An error ocurred while getting the assignments: {}", e))?;

    Ok(result)
}
