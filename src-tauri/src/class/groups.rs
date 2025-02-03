use crate::db::AppState;
use futures::TryStreamExt; // Para poder usar try_next() en los streams
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Estructura de un grupo
/// Se utiliza para mapear los datos del grupo de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: Option<i16>,
    pub grade: i16,
    pub group: String,
    pub career: Option<String>,
    pub students: Option<i16>,
}

/// Funcion para crear un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `grade` - Grado
/// * `group` - Grupo
/// * `career` - Carrera (Opcional)
/// * `students` - Cantidad de alumnos (Opcional)
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn create_group(
    pool: tauri::State<'_, AppState>,
    grade: i16,
    group: String,
    career: Option<String>,
    students: Option<i16>,
) -> Result<(), String> {
    sqlx::query(r#"INSERT INTO groups (grade, "group", career, students) VALUES (?1, ?2, ?3, ?4)"#)
        .bind(grade)
        .bind(group)
        .bind(career)
        .bind(students)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to create group, error: {}", e))?;

    println!("Group created successfully");
    Ok(())
}

/// Funcion para crear varios grupos a la vez
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `groups` - Vector de grupos
/// Retorna Ok() si todo sale exitoso de lo contrario manda un mensaje con el error
#[tauri::command]
pub async fn create_groups(
    pool: tauri::State<'_, AppState>,
    groups: Vec<Group>,
) -> Result<(), String> {
    let mut tx = pool
        .db
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction! {}", e))?;

    for g in groups {
        sqlx::query(
            r#"INSERT INTO groups(grade, "group", career, students) VALUES (?1, ?2, ?3, ?4)"#,
        )
        .bind(g.grade)
        .bind(g.group)
        .bind(g.career)
        .bind(g.students)
        .execute(&mut tx)
        .await
        .map_err(|e| format!("Error creating the group, error: {}", e))?;
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    println!("Groups created successfully");
    Ok(())
}

/// Funcion para obtener todos los grupos
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// Retorna un vector con todos los grupos
/// Se llama desde la interfaz de usuario para obtenerlos
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_groups(pool: tauri::State<'_, AppState>) -> Result<Vec<Group>, String> {
    let groups: Vec<Group> = sqlx::query_as::<_, Group>("SELECT * FROM groups")
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    Ok(groups)
}

/// Funcion para eliminar un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID del grupo:
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar un grupo
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_group(pool: tauri::State<'_, AppState>, id: i16) -> Result<(), String> {
    sqlx::query("DELETE FROM groups WHERE id = ?1")
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete group: {}", e))?;

    Ok(())
}

/// Funcion para eliminar varios grupos
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `ids` - Vector con los ID del grupo:
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar varios grupos
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_groups(pool: tauri::State<'_, AppState>, ids: Vec<i16>) -> Result<(), String> {
    for i in ids {
        delete_group(pool.clone(), i).await?;
    }

    Ok(())
}

/// Funcion para actualizar un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID del grupo
/// * `grade` - Grado
/// * `group` - Grupo
/// * `career` - Carrera (Opcional)
/// * `students` - Cantidad de alumnos (Opcional)
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para actualizar un grupo
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn update_group(
    pool: tauri::State<'_, AppState>,
    id: i16,
    grade: i16,
    group: String,
    career: Option<String>,
    students: Option<i16>,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE groups SET grade = ?1, "group" = ?2, career = ?3, students = ?4 WHERE id = ?5"#,
    )
    .bind(grade)
    .bind(group)
    .bind(career)
    .bind(students)
    .bind(id)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed to update group: {}", e))?;

    Ok(())
}
