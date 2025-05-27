use std::cmp::Ordering;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::{
    class::{
        classrooms::{get_classrooms, Classroom},
        groups::{get_group_by_id, get_group_subjects, get_groups, Group},
        subjects::{get_subject_by_id, get_subjects_with_teachers, SubjectWithTeacher},
        teachers::Teacher,
    },
    db::AppState,
};

// use super::{assignments::Assignment, constraints::constraints_satisfied};
use super::assignments::Assignment;

// TODO: Cambiar a modulos registrados
const MAX_MODULES: u8 = 9;
// TODO: Remover esta funcion cuando tengamos dias en configuracion
fn get_sorted_days() -> Vec<String> {
    vec![
        "Lunes".to_string(),
        "Martes".to_string(),
        "Miercoles".to_string(),
        "Jueves".to_string(),
        "Viernes".to_string(),
    ]
}

// Update the DAYS array to include all five weekdays
const DAYS: [&str; 5] = ["Lunes", "Martes", "Miercoles", "Jueves", "Viernes"];

// Define max_attempts constant
const MAX_ATTEMPTS: i16 = 5;

// Maximum consecutive modules for a subject (use to limit block size)
const MAX_CONSECUTIVE_MODULES: i16 = 2;

// Get the best splits for required modules (prefer small chunks of 1-2 modules)
fn get_optimal_module_splits(required_modules: i16) -> Vec<i16> {
    let mut module_blocks = Vec::new();
    let mut remaining_modules = required_modules;

    // Prefer blocks of 2 modules, but never more than MAX_CONSECUTIVE_MODULES
    while remaining_modules > 0 {
        if remaining_modules >= MAX_CONSECUTIVE_MODULES {
            module_blocks.push(MAX_CONSECUTIVE_MODULES);
            remaining_modules -= MAX_CONSECUTIVE_MODULES;
        } else {
            module_blocks.push(remaining_modules);
            remaining_modules = 0;
        }
    }

    module_blocks
}

// Add this function at the top level
fn get_teacher_availability(teacher_id: i16, schedule: &[Assignment]) -> HashMap<String, Vec<i16>> {
    // Create a map of day -> available modules for this teacher
    let mut availability: HashMap<String, Vec<i16>> = HashMap::new();

    // Initialize all days with all modules (1-9) available
    for day in DAYS.iter() {
        availability.insert(day.to_string(), (1..=9).collect());
    }

    // Remove modules that are already assigned
    for assignment in schedule {
        if assignment.teacher_id == teacher_id {
            if let Some(modules) = availability.get_mut(&assignment.day) {
                modules.retain(|&m| m != assignment.module_index);
            }
        }
    }

    availability
}

// Replace the generate_schedule function with this improved version
#[tauri::command]
pub async fn generate_schedule(
    pool: tauri::State<'_, AppState>,
) -> Result<Vec<Assignment>, String> {
    println!("Starting schedule generation...");

    // Initialize an empty schedule
    let mut schedule = Vec::new();

    // Sort subjects by priority and required modules
    println!("Sorting subjects by priority...");
    let sorted_subjects = sort_subjects_by_priority(&pool).await?;
    println!("Found {} subjects", sorted_subjects.len());

    // Sort groups by grade and complexity
    println!("Sorting groups by priority...");
    let sorted_groups = sort_groups_by_priority(&pool).await?;
    println!("Found {} groups", sorted_groups.len());

    // Three-phase scheduling approach
    // Phase 1: Reserve critical modules for each group first (1 block per critical subject)
    // This ensures all groups get a chance for critical subjects
    let mut all_group_subjects = Vec::new();

    // Gather all subjects for all groups
    for group in &sorted_groups {
        let group_subjects = get_group_subjects(&pool, group.clone()).await?;
        for subject in group_subjects {
            all_group_subjects.push((group.clone(), subject));
        }
    }

    // Create a map of teachers by subject
    let mut teachers_by_subject: HashMap<i16, Vec<Teacher>> = HashMap::new();

    println!("Phase 1: Reserving critical modules for each group");
    // First reserve one block for each critical subject (Math, Spanish, Sciences)
    // Group subjects by group_id and subject_id to avoid duplicates
    let mut reserved = HashMap::new();

    for (group, subject) in &all_group_subjects {
        // Skip if we've already reserved this subject for this group
        let key = (group.id.unwrap(), subject.id);
        if reserved.contains_key(&key) {
            continue;
        }

        // Only reserve for critical subjects (Math, Spanish, Sciences) - these are typically high modules
        if subject.required_modules.unwrap_or(2) >= 4 {
            println!(
                "Reserving critical subject {} for group {} {}",
                subject.name, group.grade, group.group
            );

            // Get teachers for this subject if we haven't already
            if !teachers_by_subject.contains_key(&subject.id) {
                let teachers = get_teachers_for_subject(&pool, subject.id).await?;
                teachers_by_subject.insert(subject.id, teachers);
            }

            // Try to reserve just the first block (2 modules)
            let block_size = 2; // Just reserve a single block
            let teachers_list = Vec::new();
            let qualified_teachers = teachers_by_subject
                .get(&subject.id)
                .unwrap_or(&teachers_list);

            // Try to find a slot for just one block
            let mut reserved_block = false;

            'day_loop: for day in DAYS.iter() {
                for starting_module in 1..=(9 - block_size) {
                    // Check if this slot is available for the group
                    if !is_schedule_available(
                        &schedule,
                        group.id.unwrap(),
                        day,
                        starting_module,
                        block_size,
                    ) {
                        continue;
                    }

                    // Find an available teacher
                    if let Some(teacher) = find_best_teacher_from_list(
                        &schedule,
                        group,
                        subject,
                        day,
                        starting_module,
                        block_size,
                        qualified_teachers,
                    ) {
                        println!(
                            "  Reserved {} for day {} module {}",
                            subject.name, day, starting_module
                        );

                        // Create assignments for this block
                        for offset in 0..block_size {
                            let module = starting_module + offset;

                            // Create a new assignment
                            let assignment = Assignment {
                                id: None,
                                group_id: group.id.unwrap(),
                                day: day.to_string(),
                                module_index: module,
                                subject_id: subject.id,
                                teacher_id: teacher.id.unwrap(),
                                classroom_id: 0,
                                subject_name: subject.name.clone(),
                                subject_shorten: subject.shorten.clone(),
                                subject_color: subject.color.clone(),
                            };

                            // Add the assignment to the schedule
                            schedule.push(assignment);
                        }

                        reserved.insert(key, true);
                        reserved_block = true;
                        break 'day_loop;
                    }
                }
            }

            if !reserved_block {
                println!(
                    "  WARNING: Could not reserve {} for group {} {}",
                    subject.name, group.grade, group.group
                );
            }
        }
    }

    println!("Phase 2: Scheduling high-priority subjects");
    // Phase 2: Complete the high-priority subjects
    for (group, subject) in &all_group_subjects {
        // Skip subjects with 2 or fewer modules for now
        if subject.required_modules.unwrap_or(2) <= 2 {
            continue;
        }

        println!(
            "Scheduling subject {} for group {} {}",
            subject.name, group.grade, group.group
        );

        // Get teachers for this subject if we haven't already
        if !teachers_by_subject.contains_key(&subject.id) {
            let teachers = get_teachers_for_subject(&pool, subject.id).await?;
            teachers_by_subject.insert(subject.id, teachers);
        }

        // Check how many modules we've already assigned for this subject
        let _key = (group.id.unwrap(), subject.id);
        let already_assigned = schedule
            .iter()
            .filter(|a| a.group_id == group.id.unwrap() && a.subject_id == subject.id)
            .count() as i16;

        // If we've already assigned all the modules for this subject, skip it
        let required_modules = subject.required_modules.unwrap_or(2);
        if already_assigned >= required_modules {
            println!("  Already assigned all {} modules", required_modules);
            continue;
        }

        // Figure out how many more modules we need to assign
        let remaining_modules = required_modules - already_assigned;
        println!("  Need to assign {} more modules", remaining_modules);

        // Create module blocks for the remaining modules
        let blocks = get_optimal_module_splits(remaining_modules);

        // Try to assign each block
        let mut all_blocks_assigned = true;

        for block_size in blocks {
            let teachers_list = Vec::new();
            let qualified_teachers = teachers_by_subject
                .get(&subject.id)
                .unwrap_or(&teachers_list);
            let mut block_assigned = false;

            'day_loop: for day in DAYS.iter() {
                for starting_module in 1..=(9 - block_size) {
                    // Check if this slot is available for the group
                    if !is_schedule_available(
                        &schedule,
                        group.id.unwrap(),
                        day,
                        starting_module,
                        block_size,
                    ) {
                        continue;
                    }

                    // Find an available teacher
                    if let Some(teacher) = find_best_teacher_from_list(
                        &schedule,
                        group,
                        subject,
                        day,
                        starting_module,
                        block_size,
                        qualified_teachers,
                    ) {
                        println!(
                            "  Assigned block of size {} for day {} module {}",
                            block_size, day, starting_module
                        );

                        // Create assignments for this block
                        for offset in 0..block_size {
                            let module = starting_module + offset;

                            // Create a new assignment
                            let assignment = Assignment {
                                id: None,
                                group_id: group.id.unwrap(),
                                day: day.to_string(),
                                module_index: module,
                                subject_id: subject.id,
                                teacher_id: teacher.id.unwrap(),
                                classroom_id: 0,
                                subject_name: subject.name.clone(),
                                subject_shorten: subject.shorten.clone(),
                                subject_color: subject.color.clone(),
                            };

                            // Add the assignment to the schedule
                            schedule.push(assignment);
                        }

                        block_assigned = true;
                        break 'day_loop;
                    }
                }
            }

            if !block_assigned {
                println!("  WARNING: Could not assign block of size {}", block_size);
                all_blocks_assigned = false;
                break;
            }
        }

        if !all_blocks_assigned {
            println!(
                "  WARNING: Could not complete all blocks for {} - continuing anyway",
                subject.name
            );
        }
    }

    println!("Phase 3: Scheduling remaining subjects for each group");
    // Phase 3: Schedule remaining subjects group by group
    for (i, group) in sorted_groups.iter().enumerate() {
        println!(
            "Processing group {}/{}: {} {}",
            i + 1,
            sorted_groups.len(),
            group.grade,
            group.group
        );

        // Get subjects for this group
        let mut group_subjects = get_group_subjects(&pool, group.clone()).await?;
        println!("Group has {} subjects", group_subjects.len());

        if group_subjects.is_empty() {
            println!(
                "No subjects found for group {} {}",
                group.grade, group.group
            );
            continue;
        }

        // Filter out subjects that have been fully scheduled
        group_subjects.retain(|subject| {
            let required_modules = subject.required_modules.unwrap_or(2);
            let assigned_modules = schedule
                .iter()
                .filter(|a| a.group_id == group.id.unwrap() && a.subject_id == subject.id)
                .count() as i16;

            assigned_modules < required_modules
        });

        println!("{} subjects remaining to schedule", group_subjects.len());

        // Get teachers for each subject
        for subject in &group_subjects {
            if !teachers_by_subject.contains_key(&subject.id) {
                let teachers = get_teachers_for_subject(&pool, subject.id).await?;
                teachers_by_subject.insert(subject.id, teachers);
            }
        }

        // Sort remaining subjects by required_modules
        group_subjects.sort_by(|a, b| {
            let a_modules = a.required_modules.unwrap_or(2);
            let b_modules = b.required_modules.unwrap_or(2);
            b_modules.cmp(&a_modules)
        });

        // Try to complete each remaining subject
        let mut critical_failure = false;

        for subject in &group_subjects {
            println!("Trying to complete subject: {}", subject.name);

            // Check how many modules we've already assigned for this subject
            let already_assigned = schedule
                .iter()
                .filter(|a| a.group_id == group.id.unwrap() && a.subject_id == subject.id)
                .count() as i16;

            // Figure out how many more modules we need to assign
            let required_modules = subject.required_modules.unwrap_or(2);
            let remaining_modules = required_modules - already_assigned;
            println!("  Need to assign {} more modules", remaining_modules);

            if remaining_modules <= 0 {
                println!("  Already fully assigned!");
                continue;
            }

            // Create module blocks for the remaining modules
            let blocks = get_optimal_module_splits(remaining_modules);

            // Try to assign each block
            let mut all_blocks_assigned = true;

            for block_size in blocks {
                let teachers_list = Vec::new();
                let qualified_teachers = teachers_by_subject
                    .get(&subject.id)
                    .unwrap_or(&teachers_list);
                let mut block_assigned = false;

                'day_loop: for day in DAYS.iter() {
                    for starting_module in 1..=(9 - block_size) {
                        // Check if this slot is available for the group
                        if !is_schedule_available(
                            &schedule,
                            group.id.unwrap(),
                            day,
                            starting_module,
                            block_size,
                        ) {
                            continue;
                        }

                        // Find an available teacher
                        if let Some(teacher) = find_best_teacher_from_list(
                            &schedule,
                            group,
                            subject,
                            day,
                            starting_module,
                            block_size,
                            qualified_teachers,
                        ) {
                            println!(
                                "  Assigned block of size {} for day {} module {}",
                                block_size, day, starting_module
                            );

                            // Create assignments for this block
                            for offset in 0..block_size {
                                let module = starting_module + offset;

                                // Create a new assignment
                                let assignment = Assignment {
                                    id: None,
                                    group_id: group.id.unwrap(),
                                    day: day.to_string(),
                                    module_index: module,
                                    subject_id: subject.id,
                                    teacher_id: teacher.id.unwrap(),
                                    classroom_id: 0,
                                    subject_name: subject.name.clone(),
                                    subject_shorten: subject.shorten.clone(),
                                    subject_color: subject.color.clone(),
                                };

                                // Add the assignment to the schedule
                                schedule.push(assignment);
                            }

                            block_assigned = true;
                            break 'day_loop;
                        }
                    }
                }

                if !block_assigned {
                    println!("  Failed to assign block of size {}", block_size);
                    all_blocks_assigned = false;
                    break;
                }
            }

            if !all_blocks_assigned {
                println!("Failed to complete subject: {}", subject.name);

                // For critical subjects (high module count), consider this a critical failure
                if required_modules >= 4 {
                    critical_failure = true;
                }
            } else {
                println!("Successfully completed subject: {}", subject.name);
            }
        }

        // Only return an error for critical failures
        if critical_failure {
            println!(
                "Critical subjects could not be assigned for group {} {}",
                group.grade, group.group
            );

            // We'll continue nonetheless - this is a best-effort approach
            // In a production system, you might want to fail here or implement backtracking
            println!(
                "WARNING: Continuing with partial schedule for group {} {}",
                group.grade, group.group
            );
        }

        println!(
            "Completed scheduling for group {} {}",
            group.grade, group.group
        );
        println!("Current schedule size: {} assignments", schedule.len());
    }

    // Save the schedule to the database
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
    println!(
        "  Getting subjects for group {} {}",
        group.grade, group.group
    );

    // Obtiene las materias para este grupo
    let group_subjects = get_group_subjects(pool, group.clone()).await?;
    println!("  Group has {} subjects", group_subjects.len());

    if group_subjects.is_empty() {
        println!(
            "  No subjects found for group {} {}",
            group.grade, group.group
        );
        return Ok(true); // No hay materias que asignar
    }

    // Crea un mapa de profesores por materia para evitar múltiples consultas
    let mut teachers_by_subject: HashMap<i16, Vec<Teacher>> = HashMap::new();
    for subject in &group_subjects {
        println!("  Getting teachers for subject: {}", subject.name);
        let teachers = get_teachers_for_subject(pool, subject.id).await?;
        println!("    Found {} qualified teachers", teachers.len());
        teachers_by_subject.insert(subject.id, teachers);
    }

    // Sort subjects by required_modules (descending) to schedule larger subjects first
    let mut subjects_to_assign = group_subjects.clone();
    subjects_to_assign.sort_by(|a, b| {
        let a_modules = a.required_modules.unwrap_or(2);
        let b_modules = b.required_modules.unwrap_or(2);
        b_modules.cmp(&a_modules)
    });

    // Asignar cada materia del grupo
    let mut all_subjects_assigned = true;

    for subject in subjects_to_assign {
        println!("  Trying to assign subject: {}", subject.name);

        // Intenta asignar esta materia
        let subject_assigned = assign_group_subject(
            pool,
            schedule,
            group,
            &subject,
            subjects,
            &teachers_by_subject,
        )
        .await?;

        if !subject_assigned {
            println!("  Failed to assign subject: {}", subject.name);
            all_subjects_assigned = false;
            // If a subject couldn't be assigned, we should consider this group a failure
            // Don't return immediately, but we will return false at the end
        } else {
            println!("  Successfully assigned subject: {}", subject.name);
        }
    }

    if !all_subjects_assigned {
        println!(
            "  Not all subjects could be assigned for group {} {}",
            group.grade, group.group
        );
        // In a production scheduler, we would probably want to fail here and backtrack
        // For now, we'll just return false to indicate failure
        return Ok(false);
    }

    // Retornamos true para continuar con el siguiente grupo
    Ok(true)
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
         WHERE ts.subject_id = ?1",
    )
    .bind(subject_id)
    .fetch_all(&pool.db)
    .await
    .map_err(|e| format!("Error al obtener profesores para la materia: {}", e))?;

    Ok(teachers)
}

// Improve the is_schedule_available function to be more precise
fn is_schedule_available(
    schedule: &[Assignment],
    group_id: i16,
    day: &str,
    starting_module: i16,
    block_size: i16,
) -> bool {
    // Check if any module in the block is already assigned for this group
    for offset in 0..block_size {
        let module = starting_module + offset;

        // Check if this module is already assigned for this group on this day
        if schedule
            .iter()
            .any(|a| a.group_id == group_id && a.day == day && a.module_index == module)
        {
            return false;
        }
    }

    // Also check if we're creating "dead modules" in the middle of the day
    // This is a heuristic to ensure we prefer contiguous blocks
    let group_modules_today: Vec<i16> = schedule
        .iter()
        .filter(|a| a.group_id == group_id && a.day == day)
        .map(|a| a.module_index)
        .collect();

    if !group_modules_today.is_empty() {
        // Check if adding this block would create isolated modules
        let would_create_isolated_module = starting_module > 1
            && !group_modules_today.contains(&(starting_module - 1))
            && group_modules_today.iter().any(|&m| m < starting_module - 1);

        // We want to discourage but not prohibit creating isolated modules
        // For now, we'll just return false if it would create an isolated module
        if would_create_isolated_module {
            // This is a soft constraint - might want to just score lower instead
            return false;
        }
    }

    return true;
}

// Improve the find_best_teacher_from_list function to be smarter about teacher selection
fn find_best_teacher_from_list(
    schedule: &Vec<Assignment>,
    _group: &Group,
    _subject: &SubjectWithTeacher,
    day: &str,
    starting_module: i16,
    block_size: i16,
    qualified_teachers: &[Teacher],
) -> Option<Teacher> {
    // Calculate a score for each qualified teacher
    let mut teacher_scores: Vec<(Teacher, i32)> = Vec::new();

    for teacher in qualified_teachers {
        // Skip teachers who aren't available for this timeslot
        if !is_teacher_available(
            schedule,
            teacher.id.unwrap(),
            day,
            starting_module,
            block_size,
        ) {
            continue;
        }

        // Calculate a score for this teacher
        let mut score = 0;

        // Prefer teachers with fewer assigned modules
        let assigned_modules = schedule
            .iter()
            .filter(|a| a.teacher_id == teacher.id.unwrap())
            .count();
        score -= assigned_modules as i32 * 10; // Lower score for teachers with more assignments

        // Prefer teachers who already have assignments on this day
        // (to minimize the number of days a teacher has to come to school)
        let already_teaching_today = schedule
            .iter()
            .any(|a| a.teacher_id == teacher.id.unwrap() && a.day == day);
        if already_teaching_today {
            score += 50;
        }

        // Prefer consecutive blocks (if the teacher has an assignment just before or after)
        let has_adjacent_block = schedule.iter().any(|a| {
            a.teacher_id == teacher.id.unwrap()
                && a.day == day
                && (a.module_index == starting_module - 1
                    || a.module_index == starting_module + block_size)
        });
        if has_adjacent_block {
            score += 100;
        }

        teacher_scores.push((teacher.clone(), score));
    }

    // Sort by score (highest first)
    teacher_scores.sort_by(|a, b| b.1.cmp(&a.1));

    // Return the teacher with the highest score, if any
    teacher_scores.first().map(|(teacher, _)| teacher.clone())
}

// Función para verificar si un horario está disponible
fn is_teacher_available(
    schedule: &[Assignment],
    teacher_id: i16,
    day: &str,
    starting_module: i16,
    block_size: i16,
) -> bool {
    // Check if the teacher is already assigned during this time slot
    for offset in 0..block_size {
        let module = starting_module + offset;
        let already_assigned = schedule
            .iter()
            .any(|a| a.teacher_id == teacher_id && a.day == day && a.module_index == module);

        if already_assigned {
            return false;
        }
    }

    true
}

// Fix assign_group_subject function to better distribute modules
async fn assign_group_subject(
    _pool: &tauri::State<'_, AppState>,
    schedule: &mut Vec<Assignment>,
    group: &Group,
    subject: &SubjectWithTeacher,
    _all_subjects: &[SubjectWithTeacher],
    teachers_by_subject: &HashMap<i16, Vec<Teacher>>,
) -> Result<bool, String> {
    println!(
        "    Assigning subject {} to group {} {}",
        subject.name, group.grade, group.group
    );

    let required_modules = subject.required_modules.unwrap_or(2);
    println!("    Required modules: {}", required_modules);

    // Use our improved module splitting function
    let module_blocks = get_optimal_module_splits(required_modules);
    println!("    Module blocks: {:?}", module_blocks);

    // Get the list of qualified teachers for this subject
    let qualified_teachers = match teachers_by_subject.get(&subject.id) {
        Some(teachers) => teachers,
        None => {
            println!(
                "    No qualified teachers found for subject {}",
                subject.name
            );
            return Ok(false);
        }
    };

    // Track all assignments we make for this subject
    let mut subject_assignments = Vec::new();

    // Try to assign each block
    for (block_idx, &block_size) in module_blocks.iter().enumerate() {
        println!("    Trying block size: {}", block_size);

        // Try to find a suitable time slot for this block
        let mut assigned = false;

        // Try each day of the week
        'day_loop: for day in DAYS.iter() {
            // Try EACH module, starting from 1 (not 2)!
            for starting_module in 1..=(9 - block_size) {
                // Check if this time slot is available for the group
                if !is_schedule_available(
                    schedule,
                    group.id.unwrap(),
                    day,
                    starting_module,
                    block_size,
                ) {
                    continue;
                }

                // Try to find a teacher who is available at this time
                if let Some(teacher) = find_best_teacher_from_list(
                    schedule,
                    group,
                    subject,
                    day,
                    starting_module,
                    block_size,
                    qualified_teachers,
                ) {
                    println!(
                        "    Found teacher {} for day {} module {}",
                        teacher.name, day, starting_module
                    );

                    // Store temporary assignments
                    let mut temp_assignments = Vec::new();

                    // Create assignments for this block
                    for offset in 0..block_size {
                        let module = starting_module + offset;

                        // Create a new assignment
                        let assignment = Assignment {
                            id: None,
                            group_id: group.id.unwrap(),
                            day: day.to_string(),
                            module_index: module,
                            subject_id: subject.id,
                            teacher_id: teacher.id.unwrap(),
                            classroom_id: 0, // We'll assign classrooms later
                            subject_name: subject.name.clone(),
                            subject_shorten: subject.shorten.clone(),
                            subject_color: subject.color.clone(),
                        };

                        // Add the assignment to our temporary list
                        temp_assignments.push(assignment);
                    }

                    // Add all assignments to the schedule
                    for assignment in temp_assignments {
                        schedule.push(assignment.clone());
                        subject_assignments.push(assignment);
                    }

                    assigned = true;
                    break 'day_loop; // Break out of both loops once we've found a slot
                }
            }
        }

        // If we couldn't assign this block, remove any assignments we've made and fail
        if !assigned {
            println!(
                "    Could not assign block {} of size {}",
                block_idx, block_size
            );

            // Remove any assignments we've made for this subject
            for assignment in &subject_assignments {
                schedule.retain(|a| {
                    !(a.group_id == assignment.group_id
                        && a.day == assignment.day
                        && a.module_index == assignment.module_index)
                });
            }

            return Ok(false);
        }
    }

    // If we've made it here, we've successfully assigned all blocks
    println!(
        "    Successfully assigned all blocks for subject {}",
        subject.name
    );
    Ok(true)
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
        println!("{:?}", assignment);
        sqlx::query(
            "INSERT INTO assignments (group_id, day, module_index, subject_id, teacher_id)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ON CONFLICT (group_id, day, module_index) DO UPDATE
        SET subject_id = excluded.subject_id, teacher_id = excluded.teacher_id",
        )
        .bind(assignment.group_id)
        .bind(&assignment.day)
        .bind(assignment.module_index - 1)
        .bind(assignment.subject_id)
        .bind(assignment.teacher_id)
        .execute(&pool.db)
        .await
        .map_err(|e| format!("Error inserting assignment: {}", e))?;
    }

    println!("All assignments saved successfully");
    Ok(())
}

// Remove or fix the incomplete assign_subject function
// If you need this function, complete it properly
/*
async fn assign_subject(
    pool: &tauri::State<'_, AppState>,
    schedule: &mut Vec<Assignment>,
    group: &Group,
    subject: &SubjectWithTeacher,
    teachers_by_subject: &HashMap<i16, Vec<Teacher>>,
    max_attempts: i16,
) -> Result<bool, String> {
    // Implement this function if needed
    Ok(true)
}
*/
