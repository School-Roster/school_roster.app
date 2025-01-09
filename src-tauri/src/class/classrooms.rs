use futures::TryStreamExt; // Para poder usar try_next() en los streams
use crate::db::AppState;
use sqlx::prelude::FromRow;
use serde::{Deserialize, Serialize};

/// Estructural salon
/// Se utiliza para mapear los datos de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Classroom {
    pub id: i16,
    pub building_id: Option<String>, // Puede ser una letra o numero entonces lo dejaremos como String
    pub building_number: i16, // Numero de aula, lo que sigue despues del building_id (ejemplo: 303)
    pub building_type: Option<String>,
    pub capacity: Option<i16>,
}

/// Funcion para crear un nuevo elemento
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `building_id` - *Identificador del edificio puede ser letra o numero
/// * `building_numer` - *Numero del aula en el edificio
/// * `building_type` - Tipo especifico del edificio (ej: Ciencias)
/// * `capacity` - Capacidad de personas en el salon
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn create_classroom(
    pool: tauri::State<'_, AppState>,
    building_id: Option<String>,
    building_number: i16,
    building_type: Option<String>,
    capacity: Option<i16>,
) -> Result<(), String> {
    sqlx::query("INSERT INTO classroom (building_number, building_id, building_type, capacity) VALUES (?1, ?2, ?3, ?4)")
        .bind(building_number)
        .bind(building_id)
        .bind(building_type)
        .bind(capacity)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to create building_id, error: {}", e))?;

    println!("building_id created successfully");
    Ok(())
}

/// Funcion para obtener todos los datos en la tabla
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// Retorna un vector con las aulas registradas
/// Se llama desde la interfaz de usuario para obtenerlos
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_classrooms(pool: tauri::State<'_, AppState>) -> Result<Vec<Classroom>, String> {
    let classrooms: Vec<Classroom> = sqlx::query_as::<_, Classroom>("SELECT * FROM classroom")
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    Ok(classrooms)
}

/// Funcion para eliminar un elemento de la base de datos
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID del elemento a eliminar
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar un elemento
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_classroom(pool: tauri::State<'_, AppState>, id: i16) -> Result<(), String> {
    sqlx::query("DELETE FROM classroom WHERE id = ?1")
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete classroom: {}", e))?;

    Ok(())
}

/// Funcion para actualizar un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `id` - ID del grupo
/// * `building_id` - *Identificador del edificio puede ser letra o numero
/// * `building_numer` - *Numero del aula en el edificio
/// * `building_type` - Tipo especifico del edificio (ej: Ciencias)
/// * `capacity` - Capacidad de personas en el salon
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para actualizar un grupo
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn update_classroom(
    pool: tauri::State<'_, AppState>,
    id: i16,
    building_number: i16,
    building_id: String,
    building_type: Option<String>,
    capacity: Option<i16>,
) -> Result<(), String> {
    sqlx::query("UPDATE classroom SET building_number = ?1, building_id = ?2, building_type = ?3, capacity= ?4 WHERE id = ?5")
        .bind(building_number)
        .bind(building_id)
        .bind(building_type)
        .bind(capacity)
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to update classroom: {}", e))?;

    Ok(())
}
