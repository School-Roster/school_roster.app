use std::collections::HashMap;

use crate::{
    class::{
        classrooms::get_classrooms,
        groups::{get_group_subjects, Group},
        subjects::{get_subjects_with_teachers, Subject},
        teachers::Teacher,
    },
    db::AppState,
};

use super::{assignments::Assignment, constraints::constraints_satisfied};

#[tauri::command]
pub fn generate_schedule(pool: tauri::State<'_, AppState>) -> Result<Vec<Assignment>, Error> {
    // Inicializar un horario vacio
    let mut schedule = Vec::new();

    // Ordena materias por prioridad y modulos requiridos
    let sorted_subjects = sort_subjects_by_priority(pool);

    // Ordena grupos por grado y complejidad
    let sorted_groups = sort_groups_by_priority(pool);

    // Por cada grupo asignamos materias (backtracking)
    for group in sorted_groups {
        if !assign_group_schedule(&mut schedule, &group, &sorted_subjects) {
            return Err(Error::NoValidScheduleFound);
        }
    }

    // Optimizar asignacion de aulas
    assign_classrooms(&mut schedule);

    Ok(schedule)
}

fn sort_subjects_by_priority(pool: tauri::State<'_, AppState>) -> Vec<Subject> {
    let mut subjects = get_subjects_with_teachers(pool);
    subjects.sort_by(|a, b| {
        let priority_cmp = b.priority.cmp(&a.priority);
        if priority_cmp == Ordering::Equal {
            b.required_modules.cmp(&a.required_modules)
        } else {
            priority_cmp
        }
    });
    subjects
}

fn sort_groups_by_priority(pool: tauri::State<'_, AppState>) -> Vec<Group> {
    let mut groups = get_groups();
    groups.sort_by(|a, b| {
        let grade_cmp = a.grade.cmp(&b.grade);
        if grade_cmp == Ordering::Equal {
            a.complexity.cmp(&b.complexity)
        } else {
            grade_cmp
        }
    });
    groups
}

fn assign_group_schedule(
    schedule: &mut Vec<Assignment>,
    group: &Group,
    subjects: &[Subject],
) -> bool {
    // Prueba todas las materias para este grupo
    for subject in get_group_subjects(group) {
        let required_modules = subject.required_modules.unwrap_or(2);

        // Intentar modulos de 2 horas
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

        // Pruebe cada tamaño de bloque
        for &block_size in &module_blocks {
            // Prueba cada combinacion de dia/modulo
            for day in get_sorted_days() {
                // Ubica los dias para preferir el inicio de semana
                // Intenta primeros modulos para evitar horas huecas
                for starting_module in 1..=(MAX_MODULES - block_size + 1) {
                    let teacher = find_best_teacher(
                        schedule,
                        group,
                        subject,
                        day,
                        starting_module,
                        block_size,
                    );

                    if let Some(teacher) = teacher {
                        // Crea la asignacion para el bloque
                        let mut success = true;
                        let original_schedule_size = schedule.len();

                        for offset in 0..block_size {
                            let assignment = Assignment {
                                id: None, // Se asigna por defecto
                                group_id: group.id.unwrap(),
                                day: day.to_string(),
                                module_index: starting_module + offset,
                                subject_id: subject.id.unwrap(),
                                teacher_id: teacher.id.unwrap(),
                                classroom_id: 0, // Lo asignamos despues
                                subject_shorten: subject.shorten.clone(),
                                subject_color: subject.color.clone(),
                            };

                            schedule.push(assignment);
                        }

                        // Intenar asignar el resto de bloques para esta materia (si aplica)
                        if module_blocks.len() > 1 {
                            let remaining_blocks = module_blocks[1..].to_vec();
                            if assign_remaining_blocks(schedule, group, subject, &remaining_blocks)
                            {
                                return true;
                            }
                        } else {
                            // Mover a la siguiente materia
                            if assign_next_subject(schedule, group, subjects, subject) {
                                return true;
                            }
                        }

                        // Si el programa llega a este punto, la asignacion no se realizo
                        // Elimina posibles errores
                        while schedule.len() > original_schedule_size {
                            schedule.pop();
                        }
                    }
                }
            }
        }
    }

    // Ninguna combinacion funciono
    false
}

fn find_best_teacher(
    schedule: &Vec<Assignment>,
    group: &Group,
    subject: &Subject,
    day: &str,
    starting_module: i16,
    block_size: i16,
) -> Option<Teacher> {
    let qualified_teachers = get_teachers_for_subject(subject.id.unwrap());

    // Calcula el puntaje de cada profesor
    let mut teacher_scores: Vec<(Teacher, i32)> = Vec::new();

    for teacher in qualified_teachers {
        // Checa las restricciones primero
        if !constraints_satisfied(
            schedule,
            group,
            day,
            starting_module,
            subject,
            &teacher,
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

        // Maestros con dias preferidos
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

        teacher_scores.push((teacher, score));
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

fn assign_classroom(
    pool: tauri::State<'_, AppState>,
    schedule: &mut Vec<Assignment>,
) -> Result<(), Error> {
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
    let all_classrooms = get_classrooms(pool)?;

    // Procesa cada carga horaria
    for ((day, module), assignment_indices) in assignments_by_time {
        // Itera cada asignacion en el intervalo
        for &idx in &assignment_indices {
            let assignment = &schedule[idx];

            // Obtener el grupo y materia de la asignacion
            let group = get_group_by_id(pool, assignment.group_id)?;
            let subject = get_subject_by_id(pool, assignment.subject_id)?;

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
                return Err(Error::NoSuitableClassroom {
                    group: group.grade.to_string() + &group.group,
                    day: day.clone(),
                    module,
                    subject: subject.name.clone(),
                });
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
