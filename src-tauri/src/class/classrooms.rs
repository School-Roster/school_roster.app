use crate::db::AppState;
use crate::util::assignments::Assignment;
use serde::{Deserialize, Serialize};
use sqlx::error::Error as SqlxError;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

/// Estructural salon
/// Se utiliza para mapear los datos de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Classroom {
    pub id: Option<i16>,
    pub building_id: Option<String>, // Puede ser una letra o numero entonces lo dejaremos como String
    pub building_number: i16, // Numero de aula, lo que sigue despues del building_id (ejemplo: 303)
    pub building_type: Option<String>,
    pub capacity: Option<i16>,
    pub availability: Option<Vec<(String, i16)>>, // Lista con (dias, modulos)
}

impl<'r> FromRow<'r, SqliteRow> for Classroom {
    fn from_row(row: &'r SqliteRow) -> Result<Self, SqlxError> {
        let availability_str: String = row.try_get("availability")?;
        let availability: Vec<(String, i16)> =
            serde_json::from_str(&availability_str).unwrap_or_default();

        Ok(Classroom {
            id: row.try_get("id")?,
            building_id: row.try_get("building_id")?,
            building_number: row.try_get("building_number")?,
            building_type: row.try_get("building_type")?,
            capacity: row.try_get("capacity")?,
            availability: Some(availability),
        })
    }
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
    let availability_json = serde_json::to_string(&cr.availability)
        .map_err(|e| format!("Failed to serialize availability: {}", e))?;
    sqlx::query(
        "
        INSERT INTO classroom (
            building_number,
            building_id,
            building_type,
            capacity,
            availability
        ) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(cr.building_number)
    .bind(cr.building_id)
    .bind(cr.building_type)
    .bind(cr.capacity)
    .bind(availability_json)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed to create building_id, error: {}", e))?;

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
        let availability_json = serde_json::to_string(&c.availability)
            .map_err(|e| format!("Failed to serialize availability: {}", e))?;
        sqlx::query(
            r#"INSERT INTO classroom (building_id, building_number, building_type, capacity, availability) VALUES (?1, ?2, ?3, ?4, ?5)"#,
        )
        .bind(c.building_id)
        .bind(c.building_number)
        .bind(c.building_type)
        .bind(c.capacity)
        .bind(availability_json)
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
        .fetch_all(&pool.db)
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
    let availability_json = serde_json::to_string(&classroom.availability)
        .map_err(|e| format!("Failed to serialize availability: {}", e))?;
    println!("{:?}", classroom);
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
    .bind(availability_json)
    .bind(classroom.id)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed to update classroom: {}", e))?;

    Ok(())
}

/// Funcion para asignar un aula a la asignacion en dia/modulo
#[tauri::command]
pub async fn assign_classroom_to_assignment(
    assignment_id: i32,
    classroom_id: i32,
    pool: tauri::State<'_, AppState>,
) -> Result<(), String> {
    sqlx::query("UPDATE assignments SET classroom_id = ?1 WHERE id = ?2")
        .bind(classroom_id)
        .bind(assignment_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error asignando aula: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn check_classroom_availability(
    classroom_id: i32,
    day: String,
    module_index: i32,
    pool: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let exists: Option<(i32,)> = sqlx::query_as(
        "SELECT 1 FROM assignments 
         WHERE classroom_id = $1 AND day = $2 AND module_index = $3
         LIMIT 1",
    )
    .bind(classroom_id)
    .bind(day)
    .bind(module_index)
    .fetch_optional(&pool.db)
    .await
    .map_err(|e| format!("Error verificando aula: {}", e))?;

    Ok(exists.is_none())
}

#[tauri::command]
pub async fn get_classroom_assignment(
    classroom_id: i32,
    day: String,
    module_index: i32,
    pool: tauri::State<'_, AppState>,
) -> Result<Option<Assignment>, String> {
    sqlx::query_as::<_, Assignment>(
        "SELECT * FROM assignments 
         WHERE classroom_id = ?1 AND day = ?2 AND module_index = ?3
         LIMIT 1",
    )
    .bind(classroom_id)
    .bind(day)
    .bind(module_index)
    .fetch_optional(&pool.db)
    .await
    .map_err(|e| format!("Error verificando aula: {}", e))
}

/// Funcion para remover un aula de dia+modulo
#[tauri::command]
pub async fn remove_classroom_assignment(
    assignment_id: i32,
    pool: tauri::State<'_, AppState>,
) -> Result<(), String> {
    sqlx::query("UPDATE assignments SET classroom_id = NULL WHERE id = ?1")
        .bind(assignment_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error removiendo aula: {}", e))?;

    Ok(())
}
