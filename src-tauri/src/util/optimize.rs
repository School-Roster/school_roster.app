// Schedule optimization function
fn optimize_schedule(schedule: &mut Vec<Assignment>) -> Result<Vec<Assignment>, Error> {
    let mut optimized = schedule.clone();
    let mut improved = true;

    // Keep optimizing until no further improvements can be made
    while improved {
        improved = false;

        // 1. Eliminate isolated modules for teachers
        improved |= optimize_teacher_isolated_modules(&mut optimized)?;

        // 2. Balance subject distribution across the week for groups
        improved |= optimize_group_subject_distribution(&mut optimized)?;

        // 3. Try to move modules to earlier in the day
        improved |= optimize_early_modules(&mut optimized)?;

        // 4. Try to consolidate classroom usage
        improved |= optimize_classroom_usage(&mut optimized)?;
    }

    Ok(optimized)
}

// Eliminate isolated teaching modules
fn optimize_teacher_isolated_modules(schedule: &mut Vec<Assignment>) -> Result<bool, Error> {
    let mut improved = false;

    // Group assignments by teacher and day
    let mut teacher_day_modules: HashMap<(i16, String), Vec<i16>> = HashMap::new();

    for assignment in schedule.iter() {
        let key = (assignment.teacher_id, assignment.day.clone());
        teacher_day_modules
            .entry(key)
            .or_insert_with(Vec::new)
            .push(assignment.module_index);
    }

    // Find isolated modules (a single module with no adjacent modules)
    for ((teacher_id, day), modules) in &teacher_day_modules {
        if modules.len() == 1 {
            let isolated_module = modules[0];

            // Find the assignment with this isolated module
            let isolated_idx = schedule
                .iter()
                .position(|a| {
                    a.teacher_id == *teacher_id
                        && a.day == *day
                        && a.module_index == isolated_module
                })
                .unwrap();

            // Find possible swap candidates (same subject and group, different day/module)
            let subject_id = schedule[isolated_idx].subject_id;
            let group_id = schedule[isolated_idx].group_id;

            // Try to find a day where this teacher already has multiple modules
            for ((other_teacher_id, other_day), other_modules) in &teacher_day_modules {
                if *other_teacher_id == *teacher_id
                    && *other_day != *day
                    && other_modules.len() >= 2
                {
                    // Try to find another teacher who can take over one of the modules on the crowded day
                    let module_to_swap = *other_modules.first().unwrap(); // Just pick the first one

                    let other_idx = schedule
                        .iter()
                        .position(|a| {
                            a.teacher_id == *other_teacher_id
                                && a.day == *other_day
                                && a.module_index == module_to_swap
                        })
                        .unwrap();

                    let other_subject_id = schedule[other_idx].subject_id;

                    // Find alternative teachers for the other subject
                    let alt_teachers = get_teachers_for_subject(other_subject_id)?;

                    for alt_teacher in alt_teachers {
                        if alt_teacher.id.unwrap() != *teacher_id {
                            // Check if this teacher is available during this slot
                            let is_available = !schedule.iter().any(|a| {
                                a.teacher_id == alt_teacher.id.unwrap()
                                    && a.day == *other_day
                                    && a.module_index == module_to_swap
                            });

                            if is_available {
                                // Swap the teacher for this module
                                schedule[other_idx].teacher_id = alt_teacher.id.unwrap();
                                improved = true;
                                break;
                            }
                        }
                    }

                    if improved {
                        break;
                    }
                }
            }

            // If we couldn't eliminate by swapping, try to move the isolated module
            if !improved {
                // Find other days/modules where both the group and teacher are free
                for module in 1..=8 {
                    // Assuming 8 modules per day
                    for day_option in ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"] {
                        if day_option == *day {
                            continue; // Skip current day
                        }

                        // Check if group is already scheduled at this time
                        let group_busy = schedule.iter().any(|a| {
                            a.group_id == group_id
                                && a.day == day_option
                                && a.module_index == module
                        });

                        // Check if teacher is already scheduled at this time
                        let teacher_busy = schedule.iter().any(|a| {
                            a.teacher_id == *teacher_id
                                && a.day == day_option
                                && a.module_index == module
                        });

                        if !group_busy && !teacher_busy {
                            // Check if this slot would be adjacent to other modules for this teacher
                            let would_be_adjacent = schedule.iter().any(|a| {
                                a.teacher_id == *teacher_id
                                    && a.day == day_option
                                    && (a.module_index == module - 1
                                        || a.module_index == module + 1)
                            });

                            if would_be_adjacent {
                                // Move the module to this new slot
                                schedule[isolated_idx].day = day_option.to_string();
                                schedule[isolated_idx].module_index = module;
                                improved = true;
                                break;
                            }
                        }
                    }

                    if improved {
                        break;
                    }
                }
            }
        }
    }

    Ok(improved)
}

// Balance subject distribution across the week for groups
fn optimize_group_subject_distribution(schedule: &mut Vec<Assignment>) -> Result<bool, Error> {
    let mut improved = false;

    // Group assignments by group and day
    let mut group_day_subjects: HashMap<(i16, String), Vec<i16>> = HashMap::new();

    for assignment in schedule.iter() {
        let key = (assignment.group_id, assignment.day.clone());
        group_day_subjects
            .entry(key)
            .or_insert_with(Vec::new)
            .push(assignment.subject_id);
    }

    // Find days with too many different subjects
    for ((group_id, day), subjects) in &group_day_subjects {
        let unique_subjects: HashSet<i16> = subjects.iter().cloned().collect();

        if unique_subjects.len() > 3 {
            // This day has too many different subjects, try to move one
            let subject_to_move = *unique_subjects.iter().next().unwrap();

            // Find assignments with this subject on this day
            let assignments_to_move: Vec<usize> = schedule
                .iter()
                .enumerate()
                .filter(|(_, a)| {
                    a.group_id == *group_id && a.day == *day && a.subject_id == subject_to_move
                })
                .map(|(idx, _)| idx)
                .collect();

            // Try to find another day with fewer subjects
            for other_day in ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"] {
                if other_day == *day {
                    continue;
                }

                let other_day_key = (*group_id, other_day.to_string());
                let other_day_subjects = group_day_subjects.get(&other_day_key).unwrap_or(&vec![]);
                let unique_other_subjects: HashSet<i16> =
                    other_day_subjects.iter().cloned().collect();

                if unique_other_subjects.len() < 2 {
                    // This day has room for another subject

                    // Check if all teachers are available on this new day/modules
                    let mut all_available = true;
                    let mut new_modules: Vec<i16> = Vec::new();

                    for idx in &assignments_to_move {
                        let assignment = &schedule[*idx];
                        let teacher_id = assignment.teacher_id;

                        // Find a free module for this teacher on the new day
                        let mut found_module = false;

                        for module in 1..=8 {
                            // Check if teacher is busy
                            let teacher_busy = schedule.iter().any(|a| {
                                a.teacher_id == teacher_id
                                    && a.day == other_day
                                    && a.module_index == module
                            });

                            // Check if group is busy
                            let group_busy = schedule.iter().any(|a| {
                                a.group_id == *group_id
                                    && a.day == other_day
                                    && a.module_index == module
                            });

                            if !teacher_busy && !group_busy {
                                new_modules.push(module);
                                found_module = true;
                                break;
                            }
                        }

                        if !found_module {
                            all_available = false;
                            break;
                        }
                    }

                    if all_available {
                        // Move all assignments to the new day
                        for (i, idx) in assignments_to_move.iter().enumerate() {
                            schedule[*idx].day = other_day.to_string();
                            schedule[*idx].module_index = new_modules[i];
                        }

                        improved = true;
                        break;
                    }
                }
            }
        }
    }

    Ok(improved)
}

// Try to move modules to earlier in the day
fn optimize_early_modules(schedule: &mut Vec<Assignment>) -> Result<bool, Error> {
    let mut improved = false;

    // Group assignments by group and day
    let mut group_day_modules: HashMap<(i16, String), Vec<i16>> = HashMap::new();

    for assignment in schedule.iter() {
        let key = (assignment.group_id, assignment.day.clone());
        group_day_modules
            .entry(key)
            .or_insert_with(Vec::new)
            .push(assignment.module_index);
    }

    // Look for gaps in the schedule (e.g., modules 1, 2, 4, 5 - missing 3)
    for ((group_id, day), mut modules) in &mut group_day_modules {
        modules.sort();

        if modules.len() >= 2 {
            for i in 0..modules.len() - 1 {
                if modules[i + 1] - modules[i] > 1 {
                    // Found a gap
                    let gap_start = modules[i] + 1;
                    let gap_end = modules[i + 1] - 1;

                    // Find assignments with later modules
                    let late_modules: Vec<usize> = schedule
                        .iter()
                        .enumerate()
                        .filter(|(_, a)| {
                            a.group_id == *group_id && a.day == *day && a.module_index > gap_end
                        })
                        .map(|(idx, _)| idx)
                        .collect();

                    if !late_modules.is_empty() {
                        // Try to move the earliest late module to fill the gap
                        let earliest_late_idx = *late_modules
                            .iter()
                            .min_by_key(|&&idx| schedule[idx].module_index)
                            .unwrap();
                        let teacher_id = schedule[earliest_late_idx].teacher_id;

                        // Check if teacher is available during the gap
                        let teacher_available = !schedule.iter().any(|a| {
                            a.teacher_id == teacher_id
                                && a.day == *day
                                && a.module_index >= gap_start
                                && a.module_index <= gap_end
                        });

                        if teacher_available {
                            // Move this module to fill the gap
                            schedule[earliest_late_idx].module_index = gap_start;
                            improved = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(improved)
}

// Try to consolidate classroom usage
fn optimize_classroom_usage(schedule: &mut Vec<Assignment>) -> Result<bool, Error> {
    let mut improved = false;

    // Group assignments by group and day
    let mut group_day_classrooms: HashMap<(i16, String), HashSet<i16>> = HashMap::new();

    for assignment in schedule.iter() {
        if assignment.classroom_id != 0 {
            let key = (assignment.group_id, assignment.day.clone());
            group_day_classrooms
                .entry(key)
                .or_insert_with(HashSet::new)
                .insert(assignment.classroom_id);
        }
    }

    // Look for groups using multiple classrooms on the same day
    for ((group_id, day), classrooms) in &group_day_classrooms {
        if classrooms.len() > 1 {
            // Get all classrooms used by this group on this day
            let classroom_ids: Vec<i16> = classrooms.iter().cloned().collect();

            // Try to consolidate to the most-used classroom
            let mut classroom_usage: HashMap<i16, usize> = HashMap::new();

            for assignment in schedule.iter() {
                if assignment.group_id == *group_id
                    && assignment.day == *day
                    && assignment.classroom_id != 0
                {
                    *classroom_usage.entry(assignment.classroom_id).or_insert(0) += 1;
                }
            }

            // Find the most-used classroom
            let most_used = classroom_usage
                .iter()
                .max_by_key(|(_, &count)| count)
                .map(|(&id, _)| id)
                .unwrap();

            // Try to move assignments to the most-used classroom
            for classroom_id in classroom_ids {
                if classroom_id != most_used {
                    let assignments_to_move: Vec<usize> = schedule
                        .iter()
                        .enumerate()
                        .filter(|(_, a)| {
                            a.group_id == *group_id
                                && a.day == *day
                                && a.classroom_id == classroom_id
                        })
                        .map(|(idx, _)| idx)
                        .collect();

                    for idx in assignments_to_move {
                        let module = schedule[idx].module_index;

                        // Check if most_used classroom is available at this time
                        let most_used_available = !schedule.iter().any(|a| {
                            a.classroom_id == most_used
                                && a.day == *day
                                && a.module_index == module
                                && a.group_id != *group_id
                        });

                        if most_used_available {
                            // Check if the subject requirements match the classroom
                            let subject_id = schedule[idx].subject_id;
                            let subject = get_subject_by_id(subject_id)?;
                            let classroom = get_classroom_by_id(most_used)?;

                            let requirements_match = match &subject.spec {
                                Some(spec) => match &classroom.building_type {
                                    Some(building_type) => building_type.contains(spec),
                                    None => false,
                                },
                                None => true,
                            };

                            if requirements_match {
                                schedule[idx].classroom_id = most_used;
                                improved = true;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(improved)
}
