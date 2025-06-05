use crate::db::AppState;
use futures::TryStreamExt; // Para poder usar try_next() en los streams
use serde::{Deserialize, Serialize};
use sqlx::error::Error as SqlxError;
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use tauri::api::dialog::blocking::FileDialogBuilder;

use crate::class::subjects::SubjectWithTeacher;

/// Estructura de un grupo
/// Se utiliza para mapear los datos del grupo de la base de datos a un objeto en Rust
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSubjects {
    pub group_id: i16,
    pub subject_id: i16,
}

impl<'r> FromRow<'r, SqliteRow> for GroupSubjects {
    fn from_row(row: &'r SqliteRow) -> Result<Self, SqlxError> {
        Ok(GroupSubjects {
            group_id: row.try_get("group_id")?,
            subject_id: row.try_get("subject_id")?,
        })
    }
}

/// Estructura de un estudiante
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub id: Option<i16>,
    pub name: String,
    pub father_lastname: String,
    pub mother_lastname: Option<String>,
    pub group_id: Option<i16>,
}

impl<'r> FromRow<'r, SqliteRow> for Student {
    fn from_row(row: &'r SqliteRow) -> Result<Self, SqlxError> {
        Ok(Student {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            father_lastname: row.try_get("father_lastname")?,
            mother_lastname: row.try_get("mother_lastname")?,
            group_id: row.try_get("group_id")?,
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
#[allow(unused)]
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

/// Funcion para conseguir las materias de todos los grupos (backend function)
/// # Argumentos
/// * `pool` - Conexion con la base datos
/// Retorna un vector GroupSubjects
// pub async fn get_all_group_subjects(
//     pool: &tauri::State<'_, AppState>,
// ) -> Result<Vec<GroupSubjects>, String> {
//     let query: Vec<GroupSubjects> = sqlx::query("SELECT group_id, subject_id FROM groups_subjects")
//         .fetch(&pool.db)
//         .await
//         .map_err(|e| format!("An error occurred while getting the assignments: {}", e))?;

//     Ok(query)
// }

/// Funcion para conseguir las materias de un grupo
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `group` - Clase del grupo
/// Retorna un vector de materias
pub async fn get_group_subjects(
    pool: &tauri::State<'_, AppState>,
    group: Group,
) -> Result<Vec<SubjectWithTeacher>, String> {
    let subjects_id: Vec<i16> =
        sqlx::query("SELECT subject_id FROM groups_subjects WHERE group_id = ?1")
            .bind(group.id)
            .fetch(&pool.db)
            .map_ok(|row| row.get::<i16, _>(0))
            .try_collect()
            .await
            .map_err(|e| format!("Failed to get subject IDs: {}", e))?;

    let mut required_subjects: Vec<SubjectWithTeacher> = Vec::new();

    for subject_id in subjects_id {
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

    Ok(required_subjects)
}

#[allow(unused)]
pub async fn get_group_by_id(
    pool: &tauri::State<'_, AppState>,
    group_id: i16,
) -> Result<Group, String> {
    let group: Group = sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE id=?1")
        .bind(group_id)
        .fetch_one(&pool.db)
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

/// Funcion para crear estudiantes
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `students` - Vector de estudiantes
/// * `group_id` - ID del grupo al que pertenecen los estudiantes
/// Retorna Ok() si todo sale exitoso de lo contrario manda un mensaje con el error
#[allow(unused)]
#[tauri::command]
pub async fn create_students(
    pool: tauri::State<'_, AppState>,
    students: Vec<Student>,
    groupid: i32,
) -> Result<usize, String> {
    // Retorna número de estudiantes insertados
    if students.is_empty() {
        return Err("No students provided".to_string());
    }

    let mut tx = pool
        .db
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    let mut count = 0;
    for student in students {
        // Validación básica
        if student.name.trim().is_empty() || student.father_lastname.trim().is_empty() {
            continue; // O podrías retornar error
        }

        let result = sqlx::query(
            "INSERT INTO students (name, father_lastname, mother_lastname, group_id) 
             VALUES (?, ?, ?, ?)",
        )
        .bind(student.name)
        .bind(student.father_lastname)
        .bind(student.mother_lastname.unwrap_or_default()) // Manejo de Option
        .bind(groupid)
        .execute(&mut tx)
        .await;

        match result {
            Ok(_) => count += 1,
            Err(e) => {
                tx.rollback().await.ok();
                return Err(format!("Error creating student: {}", e));
            }
        }
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(count)
}

// Función para obtener estudiantes por grupo
#[tauri::command]
pub async fn get_students_by_group(
    group_id: i32,
    pool: tauri::State<'_, AppState>,
) -> Result<Vec<Student>, String> {
    let students = sqlx::query_as::<_, Student>(
        r#"
        SELECT id, name, father_lastname, mother_lastname, group_id
        FROM students
        WHERE group_id = ?1
        ORDER BY father_lastname, mother_lastname, name
        "#,
    )
    .bind(group_id)
    .fetch_all(&pool.db)
    .await
    .map_err(|e| format!("Failed to fetch students: {}", e))?;

    Ok(students)
}

// Función para guardar archivo Excel con diálogo
#[tauri::command]
pub async fn save_excel_file(default_name: String) -> Result<Option<String>, String> {
    let file_path = FileDialogBuilder::new()
        .set_title("Guardar listas de estudiantes")
        .set_file_name(&default_name)
        .add_filter("Excel files", &["xlsx"])
        .save_file();

    match file_path {
        Some(path) => Ok(Some(path.to_string_lossy().to_string())),
        None => Ok(None),
    }
}
