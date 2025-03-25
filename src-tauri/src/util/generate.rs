use std::cmp::Ordering;
use std::collections::HashMap;

use crate::{
    class::{
        classrooms::{get_classrooms, Classroom},
        groups::{get_group_by_id, get_group_subjects, get_groups, Group},
        subjects::{get_subject_by_id, get_subjects_with_teachers, SubjectWithTeacher},
        teachers::Teacher,
    },
    db::AppState,
};

use super::{assignments::Assignment, constraints::constraints_satisfied};

// TODO: Cambiar a modulos registrados
const MAX_MODULES: u8 = 9;
// TODO: Remover esta funcion cuando tengamos dias en configuracion
fn get_sorted_days() -> Vec<String> {
    vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
    ]
}

#[tauri::command]
pub async fn generate_schedule(
    pool: tauri::State<'_, AppState>,
) -> Result<Vec<Assignment>, String> {
    println!("Starting schedule generation...");
    
    // Inicializar un horario vacio
    let mut schedule = Vec::new();

    // Ordena materias por prioridad y modulos requiridos
    println!("Sorting subjects by priority...");
    let sorted_subjects = sort_subjects_by_priority(&pool).await?;
    println!("Found {} subjects", sorted_subjects.len());

    // Ordena grupos por grado y complejidad
    println!("Sorting groups by priority...");
    let sorted_groups = sort_groups_by_priority(&pool).await?;
    println!("Found {} groups", sorted_groups.len());

    // Por cada grupo asignamos materias (backtracking)
    for (i, group) in sorted_groups.iter().enumerate() {
        println!("Processing group {}/{}: {} {}", i+1, sorted_groups.len(), group.grade, group.group);
        
        if !assign_group_schedule(&pool, &mut schedule, &group, &sorted_subjects).await? {
            println!("Failed to find valid schedule for group {} {}", group.grade, group.group);
            return Err(format!("No se encontro un horario valido para el grupo {} {}", group.grade, group.group));
        }
        
        println!("Successfully assigned schedule for group {} {}", group.grade, group.group);
        println!("Current schedule size: {} assignments", schedule.len());
    }

    // Temporalmente omitimos la asignación de aulas
    println!("Skipping classroom assignment for now...");
    // assign_classrooms(pool, &mut schedule).await?;
    
    println!("Schedule generation completed successfully with {} assignments", schedule.len());

    // Guardar el horario en la base de datos
    println!("Saving schedule to database...");
    save_schedule_to_database(&pool, &schedule).await?;
    println!("Schedule saved successfully!");

    Ok(schedule)
}

async fn sort_subjects_by_priority(
    pool: &tauri::State<'_, AppState>,
) -> Result<Vec<SubjectWithTeacher>, String> {
    let mut subjects = get_subjects_with_teachers(pool.clone()).await?;
    subjects.sort_by(|a, b| {
        let priority_cmp = b.priority.cmp(&a.priority);
        if priority_cmp == Ordering::Equal {
            b.required_modules.cmp(&a.required_modules)
        } else {
            priority_cmp
        }
    });
    Ok(subjects)
}

async fn sort_groups_by_priority(pool: &tauri::State<'_, AppState>) -> Result<Vec<Group>, String> {
    let mut groups = get_groups(pool.clone()).await?;
    groups.sort_by(|a, b| {
        let grade_cmp = a.0.grade.cmp(&b.0.grade);
        if grade_cmp == Ordering::Equal {
            a.0.group.cmp(&b.0.group)
        } else {
            grade_cmp
        }
    });
    Ok(groups.into_iter().map(|(group, _)| group).collect())
}

async fn assign_group_schedule(
    pool: &tauri::State<'_, AppState>,
    schedule: &mut Vec<Assignment>,
    group: &Group,
    subjects: &[SubjectWithTeacher],
) -> Result<bool, String> {
    println!("  Getting subjects for group {} {}", group.grade, group.group);
    
    // Obtiene las materias para este grupo
    let group_subjects = get_group_subjects(pool, group.clone()).await?;
    println!("  Group has {} subjects", group_subjects.len());
    
    // Crea un mapa de profesores por materia para evitar múltiples consultas
    let mut teachers_by_subject: HashMap<i16, Vec<Teacher>> = HashMap::new();
    for subject in &group_subjects {
        println!("  Getting teachers for subject: {}", subject.name);
        let teachers = get_teachers_for_subject(pool, subject.id).await?;
        println!("    Found {} qualified teachers", teachers.len());
        teachers_by_subject.insert(subject.id, teachers);
    }
    
    // Prueba todas las materias para este grupo
    for subject in group_subjects {
        println!("  Trying to assign subject: {}", subject.name);
        let required_modules = subject.required_modules.unwrap_or(2);
        println!("    Required modules: {}", required_modules);

        // Determina cómo dividir los módulos
        let module_blocks = if required_modules <= 2 {
            vec![2] // Siempre intenta los modulos de 2 horas
        } else if required_modules == 3 {
            vec![2, 1] // Intenta asignar como 2+1
        } else if required_modules == 4 {
            vec![2, 2] // Intenta asignar como 2+2
        } else {
            // Para las materias con mas de 4 modulos, dividir en 2
            let mut blocks = vec![];
            let mut remaining = required_modules;
            while remaining > 0 {
                if remaining >= 2 {
                    blocks.push(2);
                    remaining -= 2;
                } else {
                    blocks.push(1);
                    remaining -= 1;
                }
            }
            blocks
        };
        println!("    Module blocks: {:?}", module_blocks);

        // Pruebe cada tamaño de bloque
        for &block_size in &module_blocks {
            println!("    Trying block size: {}", block_size);
            
            // Prueba cada combinacion de dia/modulo
            for day in get_sorted_days() {
                // Ubica los dias para preferir el inicio de semana
                // Intenta primeros modulos para evitar horas huecas
                for starting_module in 1..=(MAX_MODULES - block_size + 1) {
                    // Obtiene los profesores para esta materia
                    let empty_teachers = Vec::new();
                    let qualified_teachers = teachers_by_subject.get(&subject.id).unwrap_or(&empty_teachers);
                    
                    if qualified_teachers.is_empty() {
                        println!("    No qualified teachers for subject {}", subject.name);
                        continue;
                    }
                    
                    // Encuentra el mejor profesor
                    let teacher = find_best_teacher_from_list(
                        schedule,
                        group,
                        &subject,
                        &day,
                        starting_module.into(),
                        block_size.into(),
                        qualified_teachers,
                    );

                    if let Some(teacher) = teacher {
                        println!("    Found teacher {} for day {} module {}", 
                                teacher.name, day, starting_module);
                        
                        // Crea la asignacion para el bloque
                        let original_schedule_size = schedule.len();

                        for offset in 0..block_size {
                            let assignment = Assignment {
                                id: None, // Se asigna por defecto
                                group_id: group.id.unwrap(),
                                day: day.to_string(),
                                module_index: (starting_module + offset).into(),
                                subject_id: subject.id,
                                teacher_id: teacher.id.expect("Teacher must have an ID"),
                                classroom_id: 0, // Lo asignamos despues
                                subject_shorten: subject.shorten.clone(),
                                subject_color: subject.color.clone(),
                            };

                            schedule.push(assignment);
                        }

                        // Intenta asignar el resto de bloques para esta materia (si aplica)
                        if module_blocks.len() > 1 {
                            // let remaining_blocks = module_blocks[1..].to_vec();
                            let remaining_blocks: Vec<i16> = module_blocks[1..].iter().map(|&x| x as i16).collect();
                            println!("    Trying to assign remaining blocks: {:?}", remaining_blocks);
                            if assign_remaining_blocks(
                                pool, 
                                schedule, 
                                group, 
                                &subject, 
                                &remaining_blocks, 
                                &teachers_by_subject
                            ).await? {
                                println!("    Successfully assigned all blocks for subject {}", subject.name);
                                return Ok(true);
                            }
                        } else {
                            // Mover a la siguiente materia
                            println!("    Moving to next subject after {}", subject.name);
                            if assign_next_subject(
                                pool, 
                                schedule, 
                                group, 
                                subjects, 
                                &subject, 
                                &teachers_by_subject
                            ).await? {
                                println!("    Successfully assigned all subjects for group");
                                return Ok(true);
                            }
                        }

                        // Si el programa llega a este punto, la asignacion no se realizo
                        // Elimina posibles errores
                        println!("    Assignment failed, backtracking...");
                        while schedule.len() > original_schedule_size {
                            schedule.pop();
                        }
                    }
                }
            }
        }
    }

    // Ninguna combinacion funciono
    println!("  Could not find valid schedule for group {} {}", group.grade, group.group);
    Ok(false)
}

// Función para obtener profesores calificados para una materia
async fn get_teachers_for_subject(
    pool: &tauri::State<'_, AppState>,
    subject_id: i16,
) -> Result<Vec<Teacher>, String> {
    // Consulta los profesores que pueden impartir esta materia
    let teachers = sqlx::query_as::<_, Teacher>(
        "SELECT t.* FROM teachers t
         JOIN teacher_subjects ts ON t.id = ts.teacher_id
         WHERE ts.subject_id = ?1"
    )
    .bind(subject_id)
    .fetch_all(&pool.db)
    .await
    .map_err(|e| format!("Error al obtener profesores para la materia: {}", e))?;
    
    Ok(teachers)
}

// Versión modificada que acepta una lista de profesores
fn find_best_teacher_from_list(
    schedule: &Vec<Assignment>,
    group: &Group,
    subject: &SubjectWithTeacher,
    day: &str,
    starting_module: i16,
    block_size: i16,
    qualified_teachers: &[Teacher],
) -> Option<Teacher> {
    // Calcula el puntaje de cada profesor
    let mut teacher_scores: Vec<(Teacher, i32)> = Vec::new();

    for teacher in qualified_teachers {
        // Checa las restricciones primero
        if !constraints_satisfied(
            schedule,
            group,
            day,
            starting_module,
            subject.clone(),
            teacher,
            block_size,
        ) {
            continue; // Salta al docente
        }

        // Calcular el puntaje basado en restricciones suaves
        let mut score = 0;

        // Maestros con dias preferidos
        if let Some(preferred_days) = &teacher.preferred_days {
            if preferred_days.contains(&day.to_string()) {
                score += 10;
            }
        }

        // Maestros con modulos preferidos
        if let Some(preferred_modules) = &teacher.preferred_modules {
            let matches = (0..block_size)
                .filter(|&offset| preferred_modules.contains(&(starting_module + offset)))
                .count();
            score += (matches as i32) * 5;
        }

        //Preferir profesores que ya tengan clases ese día (para continuidad)
        let teacher_already_has_classes_today = schedule
            .iter()
            .any(|a| a.teacher_id == teacher.id.unwrap() && a.day == day);

        if teacher_already_has_classes_today {
            // Verificar si la asignación será adyacente a las clases existentes
            let teacher_modules: Vec<i16> = schedule
                .iter()
                .filter(|a| a.teacher_id == teacher.id.unwrap() && a.day == day)
                .map(|a| a.module_index)
                .collect();

            let adjacent = teacher_modules
                .iter()
                .any(|&m| m + 1 == starting_module || m == starting_module + block_size);

            if adjacent {
                score += 20; // Mucha preferencia por bloques adyacentes
            } else {
                score += 5; // Poca preferencia por el mismo día aunque no sea adyacente
            }
        }

        // Prefer teachers with fewer assigned hours (load balancing)
        let teacher_assigned_hours = schedule
            .iter()
            .filter(|a| a.teacher_id == teacher.id.unwrap())
            .count() as i16;

        let max_hours = teacher.commisioned_hours.unwrap_or(40);
        let hours_factor = (max_hours - teacher_assigned_hours) as i32;
        score += hours_factor / 2; // Ligera preferencia por profesores menos cargados

        teacher_scores.push((teacher.clone(), score));
    }

    // Ordena la puntuacion descendente
    teacher_scores.sort_by(|a, b| b.1.cmp(&a.1));

    // Regresa el mejor profesor (si aplica)
    if teacher_scores.is_empty() {
        None
    } else {
        Some(teacher_scores[0].0.clone())
    }
}

// Función para asignar los bloques restantes de una materia
async fn assign_remaining_blocks(
    pool: &tauri::State<'_, AppState>,
    schedule: &mut Vec<Assignment>,
    group: &Group,
    subject: &SubjectWithTeacher,
    remaining_blocks: &[i16],
    teachers_by_subject: &HashMap<i16, Vec<Teacher>>,
) -> Result<bool, String> {
    if remaining_blocks.is_empty() {
        println!("      No more remaining blocks to assign");
        return Ok(true); // No hay más bloques que asignar
    }

    let block_size = remaining_blocks[0];
    println!("      Assigning remaining block of size {}", block_size);
    let next_blocks = if remaining_blocks.len() > 1 {
        &remaining_blocks[1..]
    } else {
        &[]
    };

    // Obtiene los profesores para esta materia
    let empty_teachers = Vec::new();
    let qualified_teachers = teachers_by_subject.get(&subject.id).unwrap_or(&empty_teachers);

    if qualified_teachers.is_empty() {
        println!("      No qualified teachers for remaining block");
        return Ok(false);
    }

    // Prueba cada combinación de día/módulo para el bloque actual
    for day in get_sorted_days() {
        for starting_module in 1..=(MAX_MODULES - block_size as u8 + 1) {
            let teacher = find_best_teacher_from_list(
                schedule,
                group,
                subject,
                &day,
                starting_module.into(),
                block_size.into(),
                qualified_teachers,
            );

            if let Some(teacher) = teacher {
                println!("      Found teacher {} for remaining block on day {} module {}", 
                        teacher.name, day, starting_module);
                
                // Crea la asignación para el bloque
                let original_schedule_size = schedule.len();

                for offset in 0..block_size {
                    let assignment = Assignment {
                        id: None,
                        group_id: group.id.unwrap(),
                        day: day.to_string(),
                        module_index: (starting_module + offset as u8).into(),
                        subject_id: subject.id,
                        teacher_id: teacher.id.expect("Teacher must have an ID"),
                        classroom_id: 0, // Lo asignamos después
                        subject_shorten: subject.shorten.clone(),
                        subject_color: subject.color.clone(),
                    };

                    schedule.push(assignment);
                }

                // Intenta asignar el resto de bloques
                let recursive_result = if !next_blocks.is_empty() {
                    println!("      Recursively assigning next blocks: {:?}", next_blocks);
                    Box::pin(assign_remaining_blocks(
                        pool, 
                        schedule, 
                        group, 
                        subject, 
                        next_blocks, 
                        teachers_by_subject
                    )).await?
                } else {
                    println!("      All blocks assigned successfully");
                    true
                };

                if recursive_result {
                    return Ok(true);
                }

                // Si llegamos aquí, la asignación no funcionó
                println!("      Failed to assign remaining blocks, backtracking...");
                // Elimina las asignaciones que hicimos
                while schedule.len() > original_schedule_size {
                    schedule.pop();
                }
            }
        }
    }

    // No se pudo asignar este bloque
    println!("      Could not assign remaining block of size {}", block_size);
    Ok(false)
}

// Función para asignar la siguiente materia después de asignar una materia completa
async fn assign_next_subject(
    pool: &tauri::State<'_, AppState>,
    schedule: &mut Vec<Assignment>,
    group: &Group,
    all_subjects: &[SubjectWithTeacher],
    current_subject: &SubjectWithTeacher,
    teachers_by_subject: &HashMap<i16, Vec<Teacher>>,
) -> Result<bool, String> {
    println!("      Finding next subject after {}", current_subject.name);
    
    // Obtiene las materias del grupo que aún no se han asignado
    let group_subjects = get_group_subjects(pool, group.clone()).await?;
    
    // Filtra las materias que ya están completamente asignadas
    let assigned_subjects: Vec<i16> = schedule
        .iter()
        .filter(|a| a.group_id == group.id.unwrap())
        .map(|a| a.subject_id)
        .collect();
    
    // Cuenta cuántos módulos se han asignado por materia
    let mut assigned_modules_count: HashMap<i16, i16> = HashMap::new();
    for assignment in schedule.iter().filter(|a| a.group_id == group.id.unwrap()) {
        *assigned_modules_count.entry(assignment.subject_id).or_insert(0) += 1;
    }
    
    // Encuentra la siguiente materia a asignar
    for subject in &group_subjects {
        // Salta la materia actual
        if subject.id == current_subject.id {
            continue;
        }
        
        // Verifica si la materia ya está completamente asignada
        let required_modules = subject.required_modules.unwrap_or(2);
        let assigned_modules = *assigned_modules_count.get(&subject.id).unwrap_or(&0);
        
        println!("      Subject {} has {}/{} modules assigned", 
                subject.name, assigned_modules, required_modules);
        
        if assigned_modules < required_modules {
            // Esta materia necesita más módulos
            println!("      Trying to assign subject: {}", subject.name);
            let assignment_result = Box::pin(assign_group_subject(
                pool, 
                schedule, 
                group, 
                &subject, 
                all_subjects, 
                teachers_by_subject
            )).await?;

            if assignment_result {
                println!("      Successfully assigned subject {}", subject.name);
                return Ok(true);
            }
        }
    }
    
    // Si todas las materias están asignadas, hemos terminado con este grupo
    let all_subjects_assigned = group_subjects.iter().all(|subject| {
        let required_modules = subject.required_modules.unwrap_or(2);
        let assigned_modules = *assigned_modules_count.get(&subject.id).unwrap_or(&0);
        assigned_modules >= required_modules
    });
    
    if all_subjects_assigned {
        println!("      All subjects assigned for group {} {}", group.grade, group.group);
    } else {
        println!("      Failed to assign all subjects for group {} {}", group.grade, group.group);
    }
    
    Ok(all_subjects_assigned)
}

// Función auxiliar para asignar una materia específica a un grupo
async fn assign_group_subject(
    pool: &tauri::State<'_, AppState>,
    schedule: &mut Vec<Assignment>,
    group: &Group,
    subject: &SubjectWithTeacher,
    all_subjects: &[SubjectWithTeacher],
    teachers_by_subject: &HashMap<i16, Vec<Teacher>>,
) -> Result<bool, String> {
    println!("        Assigning subject {} to group {} {}", 
            subject.name, group.grade, group.group);
    
    let required_modules = subject.required_modules.unwrap_or(2);
    println!("        Required modules: {}", required_modules);
    
    // Determina cómo dividir los módulos
    let module_blocks = if required_modules <= 2 {
        vec![2]
    } else if required_modules == 3 {
        vec![2, 1]
    } else if required_modules == 4 {
        vec![2, 2]
    } else {
        let mut blocks = vec![];
        let mut remaining = required_modules;
        while remaining > 0 {
            if remaining >= 2 {
                blocks.push(2);
                remaining -= 2;
            } else {
                blocks.push(1);
                remaining -= 1;
            }
        }
        blocks
    };
    println!("        Module blocks: {:?}", module_blocks);
    
    // Obtiene los profesores para esta materia
    let empty_teachers = Vec::new();
    let qualified_teachers = teachers_by_subject.get(&subject.id).unwrap_or(&empty_teachers);
    
    if qualified_teachers.is_empty() {
        println!("        No qualified teachers for subject {}", subject.name);
        return Ok(false);
    }
    
    // Intenta asignar el primer bloque
    for &block_size in &module_blocks {
        println!("        Trying block size: {}", block_size);
        
        for day in get_sorted_days() {
            for starting_module in 1..=(MAX_MODULES - block_size + 1) {
                let teacher = find_best_teacher_from_list(
                    schedule,
                    group,
                    subject,
                    &day,
                    starting_module.into(),
                    block_size.into(),
                    qualified_teachers,
                );
                
                if let Some(teacher) = teacher {
                    println!("        Found teacher {} for day {} module {}", 
                            teacher.name, day, starting_module);
                    
                    // Crea la asignación para el bloque
                    let original_schedule_size = schedule.len();
                    
                    for offset in 0..block_size {
                        let assignment = Assignment {
                            id: None,
                            group_id: group.id.unwrap(),
                            day: day.to_string(),
                            module_index: (starting_module + offset).into(),
                            subject_id: subject.id,
                            teacher_id: teacher.id.expect("Teacher must have an ID"),
                            classroom_id: 0,
                            subject_shorten: subject.shorten.clone(),
                            subject_color: subject.color.clone(),
                        };
                        
                        schedule.push(assignment);
                    }
                    
                    // Intenta asignar el resto de bloques
                    if module_blocks.len() > 1 {
                        // let remaining_blocks = module_blocks[1..].to_vec();
                        let remaining_blocks: Vec<i16> = module_blocks[1..].iter().map(|&x| x as i16).collect();
                        println!("        Trying to assign remaining blocks: {:?}", remaining_blocks);
                        if assign_remaining_blocks(pool, schedule, group, subject, &remaining_blocks, teachers_by_subject).await? {
                            println!("        Successfully assigned all blocks for subject {}", subject.name);
                            return Ok(true);
                        }
                    } else {
                        // No hay más bloques, pasamos a la siguiente materia
                        println!("        Moving to next subject after {}", subject.name);
                        if assign_next_subject(pool, schedule, group, all_subjects, subject, teachers_by_subject).await? {
                            println!("        Successfully assigned all subjects for group");
                            return Ok(true);
                        }
                    }
                    
                    // Si llegamos aquí, la asignación no funcionó
                    println!("        Assignment failed, backtracking...");
                    // Elimina las asignaciones que hicimos
                    while schedule.len() > original_schedule_size {
                        schedule.pop();
                    }
                }
            }
        }
    }
    
    // No se pudo asignar esta materia
    println!("        Could not assign subject {}", subject.name);
    Ok(false)
}

async fn assign_classrooms(
    pool: tauri::State<'_, AppState>,
    schedule: &mut Vec<Assignment>,
) -> Result<(), String> {
    // Agrupa las asignaciones grupales por día y módulo para procesar cada hora junta
    let mut assignments_by_time: HashMap<(String, i16), Vec<usize>> = HashMap::new();

    for (idx, assignment) in schedule.iter().enumerate() {
        let key = (assignment.day.clone(), assignment.module_index);
        assignments_by_time
            .entry(key)
            .or_insert_with(Vec::new)
            .push(idx);
    }

    // Get all available classrooms
    let all_classrooms = get_classrooms(pool.clone()).await?;

    // Procesa cada carga horaria
    for ((day, module), assignment_indices) in assignments_by_time {
        // Itera cada asignacion en el intervalo
        for &idx in &assignment_indices {
            let assignment = &schedule[idx];

            // Obtener el grupo y materia de la asignacion
            let group = get_group_by_id(&pool, assignment.group_id).await?;
            let subject = get_subject_by_id(&pool, assignment.subject_id).await?;

            // Filtra las aulas adecuadas
            let suitable_classrooms: Vec<&Classroom> = all_classrooms
                .iter()
                .filter(|classroom| {
                    // Checa si el salon ya esta asignado en ese modulo
                    let already_assigned = assignment_indices.iter().any(|&other_idx| {
                        let other = &schedule[other_idx];
                        other.classroom_id == classroom.id.unwrap() && idx != other_idx
                    });

                    if already_assigned {
                        return false;
                    }

                    // Checa capacidad del salon
                    if let Some(capacity) = classroom.capacity {
                        if let Some(students) = group.students {
                            if capacity < students {
                                return false;
                            }
                        }
                    }

                    // Checa disponibilidad del aula
                    if let Some(availability) = &classroom.availability {
                        let is_available = availability.iter().any(|(avail_day, avail_module)| {
                            *avail_day == day && *avail_module == module
                        });

                        if !is_available {
                            return false;
                        }
                    }

                    // Checa si la aula coincide con los requisitos de la materia (si aplica)
                    if let Some(spec) = &subject.spec {
                        if let Some(building_type) = &classroom.building_type {
                            if !building_type.contains(spec) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }

                    true
                })
                .collect();

            if suitable_classrooms.is_empty() {
                // No se encontro un aula adecuada
                return Err(format!("No se encontro un aula adecuada"));
                // TODO: Implementar mejor error handling
                // return Err(Error::NoSuitableClassroom {
                //     group: group.grade.to_string() + &group.group,
                //     day: day.clone(),
                //     module,
                //     subject: subject.name.clone(),
                // });
            }

            // Sistema de puntaje para encontrar la mejor solucion
            let mut scored_classrooms: Vec<(i16, i32)> = suitable_classrooms
                .iter()
                .map(|classroom| {
                    let mut score = 0;

                    // Prefiere salones con una capacidad cercana al tamano del grupo
                    if let (Some(capacity), Some(students)) = (classroom.capacity, group.students) {
                        // Evita aulas grandes para grupos pequenos, pero asegura suficiente capacidad
                        let size_match = 100 - (capacity - students).abs() as i32;
                        score += size_match;
                    }

                    // Prefiere aulas que ya esten asignadas con este grupo (minimiza movimiento)
                    let group_classrooms_today: Vec<i16> = schedule
                        .iter()
                        .filter(|a| {
                            a.group_id == group.id.unwrap() && a.day == day && a.classroom_id != 0
                        })
                        .map(|a| a.classroom_id)
                        .collect();

                    if group_classrooms_today.contains(&classroom.id.unwrap()) {
                        score += 200; // Mucha preferencia por el mismo salon
                    }

                    // TODO: Implementar mapeado de aulas aqui

                    // Prefiere salones en el mismo edificio para modulos seguidos
                    if module > 1 {
                        // Checa el modulo previo
                        let prev_classroom = schedule.iter().find(|a| {
                            a.group_id == group.id.unwrap()
                                && a.day == day
                                && a.module_index == module - 1
                                && a.classroom_id != 0
                        });

                        if let Some(prev) = prev_classroom {
                            let prev_classroom_data = all_classrooms
                                .iter()
                                .find(|c| c.id == Some(prev.classroom_id))
                                .unwrap();

                            if prev_classroom_data.building_id == classroom.building_id {
                                score += 100; // Preferencia al mismo edificio
                            }
                        }
                    }

                    // En caso de necesitar equipo (ej: Computadoras)
                    if let (Some(spec), Some(building_type)) =
                        (&subject.spec, &classroom.building_type)
                    {
                        if building_type.contains(spec) {
                            score += 150; // Strong preference for specialized rooms when needed
                        }
                    }

                    (classroom.id.unwrap(), score)
                })
                .collect();

            // Filtra el puntaje
            scored_classrooms.sort_by(|a, b| b.1.cmp(&a.1));

            // Asigna el mejor salon
            if let Some((best_classroom_id, _)) = scored_classrooms.first() {
                schedule[idx].classroom_id = *best_classroom_id;
            }
        }
    }

    Ok(())
}

// Función para guardar el horario en la base de datos
async fn save_schedule_to_database(
    pool: &tauri::State<'_, AppState>,
    schedule: &[Assignment],
) -> Result<(), String> {
    // Primero, eliminar todas las asignaciones existentes
    println!("Clearing existing assignments...");
    sqlx::query("DELETE FROM assignments")
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error clearing existing assignments: {}", e))?;
    
    println!("Inserting {} new assignments...", schedule.len());
    
    // Insertar las nuevas asignaciones
    for assignment in schedule {
        sqlx::query(
            "INSERT INTO assignments (group_id, day, module_index, subject_id, teacher_id, classroom_id) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        )
        .bind(assignment.group_id)
        .bind(&assignment.day)
        .bind(assignment.module_index)
        .bind(assignment.subject_id)
        .bind(assignment.teacher_id)
        .bind(assignment.classroom_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error inserting assignment: {}", e))?;
    }
    
    println!("All assignments saved successfully");
    Ok(())
}

