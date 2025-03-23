use crate::db::AppState;
use futures::TryStreamExt; // Para poder usar try_next() en los streams
use serde::{Deserialize, Serialize};
use sqlx::error::Error as SqlxError;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

use crate::class::subjects::SubjectWithTeacher;

/// Estructura de un grupo
/// Se utiliza para mapear los datos del grupo de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: Option<i16>,
    pub grade: i16,
    pub group: String,
    pub career: Option<String>,
    pub students: Option<i16>,
    pub max_modules_per_day: Option<i16>,
}

impl<'r> FromRow<'r, SqliteRow> for Group {
    fn from_row(row: &'r SqliteRow) -> Result<Self, SqlxError> {
        Ok(Group {
            id: row.try_get("id")?,
            grade: row.try_get("grade")?,
            group: row.try_get("group")?,
            career: row.try_get("career")?,
            students: row.try_get("students")?,
            max_modules_per_day: row.try_get("max_modules_per_day")?,
        })
    }
}

/// Funcion para crear un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `group` - Clase del grupo
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn create_group(
    pool: tauri::State<'_, AppState>,
    g: Group,
    subjects: Option<Vec<SubjectWithTeacher>>,
) -> Result<(), String> {
    let group_id: i16 = sqlx::query_scalar(
        r#"
        INSERT INTO groups (grade, "group", career, students, max_modules_per_day)
        VALUES (?1, ?2, ?3, ?4, ?5)
        RETURNING id
    "#,
    )
    .bind(g.grade)
    .bind(g.group)
    .bind(g.career)
    .bind(g.students)
    .bind(g.max_modules_per_day)
    .fetch_one(&pool.db)
    .await
    .map_err(|e| format!("Failed to create group, error: {}", e))?;

    if let Some(subjects) = subjects {
        for subject in subjects {
            // Checa si existe la materia en el grupo, regresa el id si se encuentra
            let check_groups: Option<i16> = sqlx::query_scalar(
                "
                SELECT 1 FROM groups_subjects
                WHERE group_id = ?1 AND subject_id = ?2
                ",
            )
            .bind(group_id)
            .bind(subject.id)
            .fetch_optional(&pool.db)
            .await
            .map_err(|e| format!("Error checking if subject exists on group table: {}", e))?;

            // Si no se encuentra lo asigna
            if check_groups.is_none() {
                sqlx::query("INSERT INTO groups_subjects (group_id, subject_id) VALUES (?1, ?2)")
                    .bind(group_id)
                    .bind(subject.id)
                    .fetch_optional(&pool.db)
                    .await
                    .map_err(|e| format!("Error assigning subject to group: {}", e))?;
            }
        }
    }

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
            r#"INSERT INTO groups(grade, "group", career, students, max_modules_per_day) VALUES (?1, ?2, ?3, ?4, ?5)"#,
        )
        .bind(g.grade)
        .bind(g.group)
        .bind(g.career)
        .bind(g.students)
        .bind(g.max_modules_per_day)
        .execute(&mut tx)
        .await
        .map_err(|e| format!("Error creating the group, error: {}", e))?;
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(())
}

/// Funcion para obtener todos los grupos
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// Retorna un vector con todos los grupos
/// Se llama desde la interfaz de usuario para obtenerlos
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_groups(
    pool: tauri::State<'_, AppState>,
) -> Result<Vec<(Group, Vec<SubjectWithTeacher>)>, String> {
    let groups: Vec<Group> = sqlx::query_as::<_, Group>("SELECT * FROM groups")
        .fetch_all(&pool.db)
        .await
        .map_err(|e| {
            println!("Failed to fetch groups: {}", e);
            e.to_string()
        })?;

    let mut groups_with_subjects: Vec<(Group, Vec<SubjectWithTeacher>)> = Vec::new();

    for group in groups {
        let subject_ids: Vec<i16> =
            sqlx::query("SELECT subject_id FROM groups_subjects WHERE group_id = ?1")
                .bind(group.id)
                .fetch(&pool.db)
                .map_ok(|row| row.get::<i16, _>(0))
                .try_collect()
                .await
                .map_err(|e| format!("Failed to get subject IDs: {}", e))?;

        let mut required_subjects: Vec<SubjectWithTeacher> = Vec::new();

        for subject_id in subject_ids {
            let subject_with_teacher: Option<SubjectWithTeacher> =
                sqlx::query_as::<_, SubjectWithTeacher>(
                    r#"
                SELECT
                    s.id,
                    s.name,
                    s.shorten,
                    s.color,
                    s.spec,
                    s.required_modules,
                    s.priority,
                    t.id as teacher_id,
                    t.name as teacher_name,
                    t.father_lastname as teacher_father_lastname
                FROM subjects s
                LEFT JOIN teacher_subjects ts ON s.id = ts.subject_id
                LEFT JOIN teachers t ON ts.teacher_id = t.id
                WHERE s.id = ?1
                "#,
                )
                .bind(subject_id)
                .fetch_optional(&pool.db)
                .await
                .map_err(|e| format!("Failed to fetch subject with teacher: {}", e))?;

            if let Some(subject) = subject_with_teacher {
                required_subjects.push(subject);
            }
        }

        groups_with_subjects.push((group, required_subjects));
    }

    Ok(groups_with_subjects)
}

/// Funcion para conseguir las materias de un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `group` - Clase del grupo
/// Retorna un vector de materias
pub async fn get_group_subjects(group: Group) -> Vec<SubjectWithTeacher> {
    let subjects_id = sqlx::query("SELECT subject_id FROM groups_subjects WHERE group_id = ?1")
        .bind(group.id)
        .fetch(&pool.db)
        .map_ok(|row| row.get::<i16, _>(0))
        .try_collect()
        .await
        .map_err(|e| format!("Failed to get subject IDs: {}", e))?;

    let mut required_subjects: Vec<SubjectWithTeacher> = Vec::new();

    for subject_id in subject_id {
        let subject_with_teacher: Option<SubjectWithTeacher> =
            sqlx::query_as::<_, SubjectWithTeacher>(
                r#"
                SELECT
                    s.id,
                    s.name,
                    s.shorten,
                    s.color,
                    s.spec,
                    s.required_modules,
                    s.priority,
                    t.id as teacher_id,
                    t.name as teacher_name,
                    t.father_lastname as teacher_father_lastname
                FROM subjects s
                LEFT JOIN teacher_subjects ts ON s.id = ts.subject_id
                LEFT JOIN teachers t ON ts.teacher_id = t.id
                WHERE s.id = ?1
                "#,
            )
            .bind(subject_id)
            .fetch_optional(&pool.db)
            .await
            .map_err(|e| format!("Failed to fetch subject with teacher: {}", e))?;

        if let Some(subject) = subject_with_teacher {
            required_subjects.push(subject);
        }
    }

    required_subjects
}

pub async fn get_group_by_id(
    pool: tauri::State<'_, AppState>,
    group_id: i16,
) -> Result<Group, String> {
    let group: Group = sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE id=?1")
        .bind(group_id)
        .fetch(&pool.db)
        .await
        .map_err(|e| format!("Failed to get group by id: {}", e))?;

    Ok(group)
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

    // Borrar asignaciones de horario ligadas al grupo
    sqlx::query("DELETE FROM assignments WHERE group_id = ?1")
        .bind(id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to delete group assignment: {}", e))?;

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
/// * `group` - Clase del grupo
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para actualizar un grupo
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn update_group(
    pool: tauri::State<'_, AppState>,
    g: Group,
    subjects: Option<Vec<SubjectWithTeacher>>,
) -> Result<(), String> {
    sqlx::query(
        r#"UPDATE groups SET grade = ?1, "group" = ?2, career = ?3, students = ?4, max_modules_per_day = ?5 WHERE id = ?6"#,
    )
    .bind(g.grade)
    .bind(g.group)
    .bind(g.career)
    .bind(g.students)
    .bind(g.max_modules_per_day)
    .bind(g.id)
    .execute(&pool.db)
    .await
    .map_err(|e| format!("Failed to update group: {}", e))?;

    if let Some(subjects) = subjects {
        // Eliminar las materias del grupo si existian
        sqlx::query("DELETE FROM groups_subjects WHERE group_id = ?1")
            .bind(g.id)
            .execute(&pool.db)
            .await
            .map_err(|e| format!("Failed to delete group subject: {}", e))?;
        for subject in subjects {
            // Agrega materia al grupo
            sqlx::query("INSERT INTO groups_subjects (group_id, subject_id) VALUES (?1, ?2)")
                .bind(g.id)
                .bind(subject.id)
                .fetch_optional(&pool.db)
                .await
                .map_err(|e| format!("Failed to assign the subject to existed group: {}", e))?;
        }
    }

    Ok(())
}
