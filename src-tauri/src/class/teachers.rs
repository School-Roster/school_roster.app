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
}

/// Estructura de una relacion profesor-materia
/// Se utiliza para mapear los datos de una relacion profesor-materia de la base de datos a un objeto en Rust
// pub struct TeacherSubject {
//     pub teacher_id: i16,
//     pub subject_id: i16,
// }

/// Funcion para agregar un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `name` - Nombre del profesor
/// * `father_lastname` - Apellido paterno del profesor
/// * `mother_lastname` - Apellido materno del profesor (opcional puede ser nulo)
/// * `email` - Correo electronico del profesor (opcional puede ser nulo)
/// * `phone` - Telefono del profesor (opcional puede ser nulo)
/// * `degree` - Grado academico del profesor (opcional puede ser nulo)
/// * `commisioned_hours` - Total de horas comisionadas del profesor (opcional puede ser nulo)
/// * `active_hours` - Total de horas activas del profesor (opcional puede ser nulo)
/// * `performance` - Desempeño del profesor (opcional puede ser nulo)
/// Retorna un resultado vacio si la operacion fue exitosa
#[allow(dead_code, unused)]
#[tauri::command(rename_all = "snake_case")]
pub async fn add_teacher(
    pool: tauri::State<'_, AppState>,
    name: String,
    father_lastname: String,
    mother_lastname: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    degree: Option<String>,
    commisioned_hours: Option<i16>,
    active_hours: Option<i16>,
    performance: Option<i16>,
) -> Result<(), String> {
    sqlx::query("
        INSERT INTO teachers (
            name,
            father_lastname,
            mother_lastname,
            email,
            phone,
            degree,
            commisioned_hours,
            active_hours,
            performance
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    )
        .bind(name)
        .bind(father_lastname)
        .bind(mother_lastname)
        .bind(email)
        .bind(phone)
        .bind(degree)
        .bind(commisioned_hours)
        .bind(active_hours)
        .bind(performance)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to create teacher: {}", e))?;

    println!("Teacher created successfully");

    Ok(())
}

/// Funcion para obtener todos los profesores
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// Retorna un vector con todos los profesores
/// Se llama desde la interfaz de usuario para obtener todos los profesores
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_all_teachers(pool: tauri::State<'_, AppState>) -> Result<Vec<Teacher>, String> {
    let teachers: Vec<Teacher> = sqlx::query_as::<_, Teacher>("SELECT * FROM teachers")
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    Ok(teachers)
}

/// Funcion para agregar una materia a un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `teacher_id` - ID del profesor
/// * `subject_id` - ID de la materia
/// Retorna un resultado vacio si la operacion fue exitosa
/// Se llama desde la interfaz de usuario para agregar una materia a un profesor puede ser desde la vista de profesores.
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn attach_subject(
    pool: tauri::State<'_, AppState>,
    teacher_id: i16,
    subject_id: i16,
) -> Result<(), String> {
    sqlx::query("
        INSERT INTO teacher_subjects (teacher_id, subject_id)
        VALUES (?1, ?2)"
    )
        .bind(teacher_id)
        .bind(subject_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Failed to attach subject to teacher: {}", e))?;

    println!("Subject attached to teacher successfully");

    Ok(())
}

/// Funcion para obtener todas las materias de un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `teacher_id` - ID del profesor
/// Retorna un vector con todas las materias de un profesor
/// Se llama desde la interfaz de usuario para obtener todas las materias de un profesor
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_teacher_subjects(
    pool: tauri::State<'_, AppState>,
    teacher_id: i16,
) -> Result<Vec<Subject>, String> {
    let subjects: Vec<Subject> = sqlx::query_as::<_, Subject>("
        SELECT s.id, s.name, s.description, s.credits, s.semester
        FROM subjects s
        JOIN teacher_subjects ts ON s.id = ts.subject_id
        WHERE ts.teacher_id = ?1
    ")
        .bind(teacher_id)
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    Ok(subjects)
}

/// Funcion para obtener todas las materias de un profesor
/// # Argumentos
/// * `pool` - Conexion a la base de datos
/// * `teacher_id` - ID del profesor
/// Retorna un vector con todas las materias de un profesor
/// Se llama desde el grid en el frontend para obtener todas las materias que tengan asignadas un profesor
#[allow(dead_code, unused)]
#[tauri::command]
pub async fn get_teacher_subjects_grid(
    pool: tauri::State<'_, AppState>,
    teacher_id: i16,
) -> Result<Vec<Subject>, String> {
    let subjects: Vec<Subject> = sqlx::query_as::<_, Subject>("
        SELECT s.id, s.name, s.description, s.credits, s.semester
        FROM subjects s
        JOIN teacher_subjects ts ON s.id = ts.subject_id
        WHERE ts.teacher_id = ?1
    ")
        .bind(teacher_id)
        .fetch(&pool.db)
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    Ok(subjects)
}
