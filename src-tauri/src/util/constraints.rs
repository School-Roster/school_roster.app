/// Funcion que maneja las restricciones del algoritmo
pub fn constraints_satisfied(
    schedule: &Vec<Assignment>,
    group: &Group,
    day: &str,
    module_index: i16,
    subject: &Subject,
    teacher: &Teacher,
    proposed_modules_count: i16, // How many consecutive modules we're trying to assign
) -> bool {
    // === Restricciones del grupo ===
    // Checa el maximo de 4 modulos por dia
    let group_modules_this_day = schedule
        .iter()
        .filter(|a| a.group_id == group.id.unwrap() && a.day == day)
        .count() as i16;

    if group_modules_this_day + proposed_modules_count > 4 {
        return false; // Excede 4 modulos
    }

    // Checa si la materia esta asignada al dia en este grupo
    let subject_already_on_day = schedule.iter().any(|a| {
        a.group_id == group.id.unwrap() && a.day == day && a.subject_id == subject.id.unwrap()
    });

    if subject_already_on_day {
        return false; // La materia ya esta asignada al dia
    }

    // Checa "modulos muertos" - preferentemente los primeros
    // Se implementa con un sistema de puntuación en lugar de una restriccion
    // NOTA: Se maneja en el orden de seleccion del modulo

    // === Restricciones del docente ===
    // Checa si el profesor ya esta asignado a un modulo
    for m in module_index..(module_index + proposed_modules_count) {
        if schedule
            .iter()
            .any(|a| a.teacher_id == teacher.id.unwrap() && a.day == day && a.module_index == m)
        {
            return false; // Teacher already busy during one of these modules
        }
    }

    // Checar si el profesor ha alcanzado el maximo de horas
    let teacher_assigned_modules = schedule
        .iter()
        .filter(|a| a.teacher_id == teacher.id.unwrap())
        .count() as i16;

    if let Some(hours) = teacher.commisioned_hours {
        if teacher_assigned_modules + proposed_modules_count > hours {
            return false; // Excederia las horas comisionadas del docente
        }
    }

    // Checa por bloques consecutivos asignados (el profesor tiene clases seguidas)
    // Aplica "soft constraint" para afectar el 'score'
    let teacher_modules_this_day: Vec<i16> = schedule
        .iter()
        .filter(|a| a.teacher_id == teacher.id.unwrap() && a.day == day)
        .map(|a| a.module_index)
        .collect();

    // Si el profesor ya tiene clases ese dia, verifica si la nueva asignacion crea huecos
    if !teacher_modules_this_day.is_empty() {
        // Checa si la nueva asignacion crea un hueco de 1 modulo
        // (which is an undesirable "dead module")
        let creates_dead_module = teacher_modules_this_day
            .iter()
            .any(|&m| (m == module_index + proposed_modules_count + 1) || (m + 1 == module_index));

        if creates_dead_module {
            // No es ideal, pero no es un rechazo estrictio.
            // TODO: Se podria implementar en un sistema de puntuacion, por ahora asi.
            return false;
        }

        // Verifica si se crea mas de un espacio de modulo muerto
        let creates_multiple_dead_modules = teacher_modules_this_day
            .iter()
            .any(|&m| (m > module_index + proposed_modules_count + 1) || (m + 1 < module_index));

        if creates_multiple_dead_modules {
            return false; // Muchos modulos muertos es una restriccion estricta
        }
    }

    true
}
