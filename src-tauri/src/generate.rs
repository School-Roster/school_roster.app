fn generate_schedule() -> Result<Vec<Assignment>, Error> {
    // Inicializar un horario vacio
    let mut schedule = Vec::new();

    // Ordena materias por prioridad y modulos requiridos
    let sorted_subjects = sort_subjects_by_priority();

    // Ordena grupos por grado y complejidad
    let sorted_groups = sort_groups_by_priority();

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

fn assign_group_schedule(
    schedule: &mut Vec<Assignment>,
    group: &Group,
    subjects: &[Subject],
) -> bool {
    // Prueba todas las materias para este grupo
    for subject in get_group_subjects(group.id.unwrap(), subjects) {
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

    // Calculate a score for each teacher
    let mut teacher_scores: Vec<(Teacher, i32)> = Vec::new();

    for teacher in qualified_teachers {
        // Check hard constraints first
        if !constraints_satisfied(
            schedule,
            group,
            day,
            starting_module,
            subject,
            &teacher,
            block_size,
        ) {
            continue; // Skip this teacher
        }

        // Calculate score based on soft constraints
        let mut score = 0;

        // Prefer teachers with preferred days that match
        if let Some(preferred_days) = &teacher.preferred_days {
            if preferred_days.contains(&day.to_string()) {
                score += 10;
            }
        }

        // Prefer teachers with preferred modules that match
        if let Some(preferred_modules) = &teacher.preferred_modules {
            let matches = (0..block_size)
                .filter(|&offset| preferred_modules.contains(&(starting_module + offset)))
                .count();
            score += (matches as i32) * 5;
        }

        // Prefer teachers who already have classes on this day (for continuity)
        let teacher_already_has_classes_today = schedule
            .iter()
            .any(|a| a.teacher_id == teacher.id.unwrap() && a.day == day);

        if teacher_already_has_classes_today {
            // Check if assignment would be adjacent to existing classes
            let teacher_modules: Vec<i16> = schedule
                .iter()
                .filter(|a| a.teacher_id == teacher.id.unwrap() && a.day == day)
                .map(|a| a.module_index)
                .collect();

            let adjacent = teacher_modules
                .iter()
                .any(|&m| m + 1 == starting_module || m == starting_module + block_size);

            if adjacent {
                score += 20; // Strong preference for adjacent blocks
            } else {
                score += 5; // Some preference for same day even if not adjacent
            }
        }

        // Prefer teachers with fewer assigned hours (load balancing)
        let teacher_assigned_hours = schedule
            .iter()
            .filter(|a| a.teacher_id == teacher.id.unwrap())
            .count() as i16;

        let max_hours = teacher.commisioned_hours.unwrap_or(40);
        let hours_factor = (max_hours - teacher_assigned_hours) as i32;
        score += hours_factor / 2; // Slight preference for less loaded teachers

        teacher_scores.push((teacher, score));
    }

    // Sort by score descending
    teacher_scores.sort_by(|a, b| b.1.cmp(&a.1));

    // Return the best teacher if any
    if teacher_scores.is_empty() {
        None
    } else {
        Some(teacher_scores[0].0.clone())
    }
}
