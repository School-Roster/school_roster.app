fn generate_schedule(
    groups: Vec<Group>,
    teachers: Vec<Teacher>,
    classrooms: Vec<Classroom>,
) -> Vec<Assignment> {
    let mut schedule = initialize_schedule(groups.len(), DAYS.len(), MODULES_PER_DAY);
    let sorted_groups = sort_groups_by_priority(groups);
    let mut assignments = Vec::new();

    for group in sorted_groups {
        let sorted_subjects = sort_subjects_by_priority(group.required_subjects);

        for subject in sorted_subjects {
            let required_modules = subject.required_modules;
            let available_slots = find_available_slots(&schedule, group.id, required_modules);

            for slot in available_slots {
                let teacher = find_available_teacher(
                    &teachers,
                    slot.day,
                    slot.module_index,
                    required_modules,
                );
                let classroom = find_available_classroom(
                    &classrooms,
                    slot.day,
                    slot.module_index,
                    group.students,
                );

                if teacher.is_some() && classroom.is_some() {
                    let assignment = Assignment {
                        group_id: group.id,
                        day: slot.day,
                        module_index: slot.module_index,
                        subject_id: subject.id,
                        teacher_id: teacher.unwrap().id,
                        classroom_id: classroom.unwrap().id,
                        subject_shorten: subject.shorten,
                        subject_color: subject.color,
                    };

                    schedule[group.id][slot.day][slot.module_index] = Some(assignment.clone());
                    assignments.push(assignment);
                    break;
                }
            }
        }
    }

    optimize_schedule(&mut schedule);
    assignments
}
