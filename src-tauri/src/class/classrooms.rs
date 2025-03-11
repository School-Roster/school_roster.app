use crate::db::AppState;
use futures::TryStreamExt; // Para poder usar try_next() en los streams
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Estructural salon
/// Se utiliza para mapear los datos de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Classroom {
    pub id: Option<i16>,
    pub building_id: Option<String>, // Puede ser una letra o numero entonces lo dejaremos como String
    pub building_number: i16, // Numero de aula, lo que sigue despues del building_id (ejemplo: 303)
    pub building_type: Option<String>,
    pub capacity: Option<i16>,
    pub availability: bool,
}

/// Funcion para crear un nuevo elemento
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `cr` - Clase del aula
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn create_classroom(
    pool: tauri::State<'_, AppState>,
    cr: Classroom,
) -> Result<(), String> {
    sqlx::query(
        "
        INSERT INTO classroom (building_number, building_id, building_type, capacity, availability)
        VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(cr.building_number)
    .bind(cr.building_id)
    .bind(cr.building_type)
    .bind(cr.capacity)
    .bind(cr.availability)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed to create building_id, error: {}", e))?;

    println!("building_id created successfully");
    Ok(())
}

/// Funcion para crear varios elementos a la vez
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `classrooms` - Vector de grupos
/// Retorna Ok() si todo sale exitoso de lo contrario manda un mensaje con el error
#[tauri::command]
pub async fn create_classrooms(
    pool: tauri::State<'_, AppState>,
    classroom: Vec<Classroom>,
) -> Result<(), String> {
    let mut tx = pool
        .db
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction! {}", e))?;

    for c in classroom {
        println!("Aula: {:?}", c);
        sqlx::query(
            r#"INSERT INTO classroom (building_id, building_number, building_type, capacity, availability) VALUES (?1, ?2, ?3, ?4, ?5)"#,
        )
        .bind(c.building_id)
        .bind(c.building_number)
        .bind(c.building_type)
        .bind(c.capacity)
        .bind(c.availability)
        .execute(&mut tx)
        .await
        .map_err(|e| format!("Error creating the classroom, error: {}", e))?;
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

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

/// Funcion para eliminar varios elementos de la base de datos
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `ids` - ID del elemento a eliminar
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para eliminar varios elementos
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn delete_classrooms(
    pool: tauri::State<'_, AppState>,
    ids: Vec<i16>,
) -> Result<(), String> {
    for i in ids {
        delete_classroom(pool.clone(), i).await?;
    }
    Ok(())
}

/// Funcion para actualizar un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `classroom` - Clase del aula
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para actualizar un grupo
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn update_classroom(
    pool: tauri::State<'_, AppState>,
    classroom: Classroom,
) -> Result<(), String> {
    sqlx::query(
        "UPDATE classroom SET
            building_number = ?1,
            building_id = ?2,
            building_type = ?3,
            capacity = ?4,
            availability = ?5
        WHERE id = ?6",
    )
    .bind(classroom.building_number)
    .bind(classroom.building_id)
    .bind(classroom.building_type)
    .bind(classroom.capacity)
    .bind(classroom.availability)
    .bind(classroom.id)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed to update classroom: {}", e))?;

    Ok(())
}
