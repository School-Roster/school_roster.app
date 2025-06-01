use crate::{
    class::{
        classrooms::{create_classrooms, get_classrooms, Classroom},
        groups::{create_groups, get_groups, Group, GroupSubjects},
        subjects::{
            create_subjects, get_subjects, get_subjects_with_teachers, link_subject_to_teacher,
            Subject, SubjectWithTeacher,
        },
        teachers::{create_teachers, get_all_teachers, Teacher},
    },
    db::{AppState, DB_NAME},
    util::assignments::save_assignment,
};
use bincode;
// use futures::{FutureExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use tauri::{api::dialog, App, Manager, Window};
use thiserror::Error;

use super::assignments::{get_all_assignments, Assignment};

#[derive(Debug, Error)]
pub enum ScheduleFileError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("Invalid file format")]
    InvalidFormat,

    #[error("Unsupported version: {0}")]
    UnsuportedVersion(u16),
}

// Alias del resultado
type ScheduleResult<T> = std::result::Result<T, ScheduleFileError>;

// File format header
// Header del formato de archivo
#[derive(Serialize, Deserialize, Debug)]
struct ScheduleFileHeader {
    magic: [u8; 4],  // "SCHA" in ASCII
    version: u16,    // File format version
    created_at: i64, // UTC timestamp
    checksum: u32,   // Simple checksum for integrity verification
}

// Tablas que se van a exportar en el archivo
#[derive(Serialize, Deserialize, Debug)]
struct ScheduleData {
    subjects: Vec<Subject>,
    teachers: Vec<Teacher>,
    teacher_subjects: Vec<SubjectWithTeacher>,
    groups: Vec<Group>,
    classrooms: Vec<Classroom>,
    assignments: Vec<Assignment>,
    // group_subjects: Vec<GroupSubjects>,
}

fn simple_checksum(data: &[u8]) -> u32 {
    data.iter().fold(0u32, |acc, &x| acc.wrapping_add(x as u32))
}

/// Funcion para exportar el archivo (.roster)
#[tauri::command]
pub async fn export_file(
    _handle: tauri::AppHandle,
    pool: tauri::State<'_, AppState>,
    _window: Window,
) -> Result<(), String> {
    // Selector de archivos del usuario
    // Utilizar canal para comunicar callback y async
    let (tx, rx) = std::sync::mpsc::channel();

    // TODO: Hacer un nombre predeterminado utilizando nombre de escuela
    dialog::FileDialogBuilder::new()
        .add_filter("Roster Files", &["roster"])
        .set_title("Guardar datos")
        .save_file(move |file_path| {
            tx.send(file_path).unwrap();
        });

    // Esperar al usuario
    let file_path = match rx.recv().unwrap() {
        Some(path) => path,
        None => return Err("Cancelado por el usuario".to_string()),
    };

    // Asegurarse de que la extension si es '.roster'
    let file_path = if file_path.extension().is_none() {
        file_path.with_extension("roster")
    } else {
        file_path
    };

    // Convierte el error personalizado a un String para que tauri lo pueda manejar
    match export_file_impl(pool, &file_path).await {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Export error: {}", e)),
    }
}

// Implementacion de la funcion con errores personalizados
async fn export_file_impl(
    pool: tauri::State<'_, AppState>,
    output_path: &PathBuf,
) -> ScheduleResult<()> {
    let subjects = get_subjects(pool.clone())
        .await
        .map_err(|_| ScheduleFileError::DatabaseError(sqlx::Error::RowNotFound))?;

    let teachers = get_all_teachers(pool.clone())
        .await
        .map_err(|e| ScheduleFileError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?
        .into_iter()
        .map(|(t, _)| t)
        .collect();

    let teacher_subjects = get_subjects_with_teachers(pool.clone())
        .await
        .map_err(|_| ScheduleFileError::DatabaseError(sqlx::Error::RowNotFound))?;

    let groups = get_groups(pool.clone())
        .await
        .map_err(|e| ScheduleFileError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?
        .into_iter()
        .map(|(g, _)| g)
        .collect();

    let classrooms = get_classrooms(pool.clone())
        .await
        .map_err(|_| ScheduleFileError::DatabaseError(sqlx::Error::RowNotFound))?;

    let assignments = get_all_assignments(pool.clone())
        .await
        .map_err(|_| ScheduleFileError::DatabaseError(sqlx::Error::RowNotFound))?;
    // let group_subjects = get_all_group_subjects(pool.clone()).await?;

    // Package data
    let data = ScheduleData {
        subjects,
        teachers,
        teacher_subjects,
        groups,
        classrooms,
        assignments,
        // group_subjects,
    };

    // Serializamos los datos utilizando bincode
    let serialized_data = bincode::serialize(&data)?;

    let checksum = simple_checksum(&serialized_data);

    let header = ScheduleFileHeader {
        magic: *b"SCHA", // Bytes para identificar nuestra formato
        version: 1,      // Version inicial
        created_at: chrono::Utc::now().timestamp(),
        checksum,
    };
    let serialized_header = bincode::serialize(&header)?;

    // Crear el archivo
    let mut file = File::create(output_path)?;
    file.write_all(&serialized_header)?;
    file.write_all(&serialized_data)?;

    Ok(())
}

/// Funcion para exportar el archivo (.roster)
#[tauri::command]
pub async fn import_file(
    handle: tauri::AppHandle,
    pool: tauri::State<'_, AppState>,
    _window: Window,
) -> Result<(), String> {
    // Selector de archivos del usuario
    // Utilizar canal para comunicar callback y async
    let (tx, rx) = std::sync::mpsc::channel();

    // TODO: Hacer un nombre predeterminado utilizando nombre de escuela
    dialog::FileDialogBuilder::new()
        .add_filter("Roster Files", &["roster"])
        .set_title("Abrir archivo de horario")
        .pick_file(move |file_path| {
            tx.send(file_path).unwrap();
        });

    // Esperar al usuario
    let file_path = match rx.recv().unwrap() {
        Some(path) => path,
        None => return Err("Cancelado por el usuario".to_string()),
    };

    // Asegurarse de que la extension si es '.roster'
    let file_path = if file_path.extension().is_none() {
        file_path.with_extension("roster")
    } else {
        file_path
    };

    // Convierte el error personalizado a un String para que tauri lo pueda manejar
    match import_file_impl(pool, handle, &file_path).await {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Export error: {}", e)),
    }
}

// Implementacion de la funcion con errores personalizados
async fn import_file_impl(
    pool: tauri::State<'_, AppState>,
    handle: tauri::AppHandle,
    file_path: &PathBuf,
) -> ScheduleResult<()> {
    // let mut file = File::open(file_path);
    let file = File::open(file_path);
    let mut buffer = Vec::new();

    file?.read_to_end(&mut buffer)?;

    // Verifica el header
    if buffer.len() < std::mem::size_of::<ScheduleFileHeader>() {
        return Err(ScheduleFileError::InvalidFormat);
    }

    let header_size = bincode::serialized_size(&ScheduleFileHeader {
        magic: *b"SCHA",
        version: 0,
        created_at: 0,
        checksum: 0,
    })? as usize;

    let header: ScheduleFileHeader = bincode::deserialize(&buffer[..header_size])?;

    // Verifica los bytes
    if &header.magic != b"SCHA" {
        return Err(ScheduleFileError::InvalidFormat);
    }

    // Checa la version
    if header.version > 1 {
        return Err(ScheduleFileError::UnsuportedVersion(header.version));
    }

    // Checksum
    let data_part = &buffer[header_size..];
    let computed_checksum = simple_checksum(data_part);
    if computed_checksum != header.checksum {
        return Err(ScheduleFileError::InvalidFormat);
    };

    // Deserealizar los datos
    let data: ScheduleData = bincode::deserialize(data_part)?;

    // Base de datos utilizada
    // Obtiene la ruta de la aplicacion.
    let mut db_path = handle.path_resolver().app_data_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to get data directory")
    })?;

    db_path.push(&DB_NAME);

    // Antes de continuar, hacer un backup (cada vez que se abre un archivo se sobrescribira el backup anterior)
    let backup_path = format!("{}.backup", db_path.display());
    fs::copy(db_path, backup_path)?;

    // Importar los datos en la base de datos
    clear_data(pool.clone()).await;

    create_subjects(pool.clone(), data.subjects).await;
    create_teachers(pool.clone(), data.teachers).await;
    link_subject_to_teacher(pool.clone(), data.teacher_subjects).await;
    create_groups(pool.clone(), data.groups).await;
    create_classrooms(pool.clone(), data.classrooms).await;
    for assignment in data.assignments {
        save_assignment(
            pool.clone(),
            assignment.group_id.into(),
            assignment.day.as_str(),
            assignment.module_index.into(),
            assignment.subject_id.into(),
            assignment.teacher_id.into(),
        )
        .await;
    }

    Ok(())
}

// WARNING
/// Funcion para eliminar todos los datos registrados en el programa
#[tauri::command]
pub async fn delete_all_data(pool: tauri::State<'_, AppState>) -> Result<(), String> {
    clear_data(pool).await;
    Ok(())
}

/// Funcion que limpia todos los datos previamente guardados (solo las tablas usadas en el importe)
// BUG: Por alguna razon no se puede utilizar '?', entonces los mensajes de WARNING en el compilador no se pueden arreglar.
#[allow(unused_must_use)]
async fn clear_data(pool: tauri::State<'_, AppState>) -> Result<(), String> {
    println!("Clearing previous data...");

    sqlx::query("DELETE FROM group_subjects")
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error while deleting group_subjects: {}", e));

    sqlx::query("DELETE FROM teacher_subjects")
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error while deleting teacher_subjects: {}", e));

    sqlx::query("DELETE FROM subjects")
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error while deleting subjects: {}", e));

    sqlx::query("DELETE FROM teachers")
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error while deleting teachers: {}", e));

    sqlx::query("DELETE FROM groups")
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error while deleting groups: {}", e));

    sqlx::query("DELETE FROM classrooms")
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error while deleting classrooms: {}", e));

    sqlx::query("DELETE FROM assignments")
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error while deleting assignments: {}", e));

    println!("Done!");
    Ok(())
}
