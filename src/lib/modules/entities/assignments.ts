import { addNotification } from "$lib/stores/notificationsStore";
import { invoke } from "@tauri-apps/api";
import { writable, type Writable } from "svelte/store";
import { derived } from "svelte/store";

/**
  * Interfaz para los datos de los grupos
  * @property {number} group_id - Identificador del grupo
  * @property {string} day - Dia asignado
  * @property {number} module_index - Espacio (modulo)
  * @property {number} subject_id - Id de la materia
  */
export interface AssignmentItem {
  id: number;
  group_id: number,
  day: string,
  module_index: number,
  subject_id: number,
  teacher_id: number,
  classroom_id: number,
  subject_name: string,
  subject_shorten: string,
  subject_color: string
}

// Mantener O(1)
export const assignmentsStore: Writable<Map<any, any>> = writable(new Map());

export async function loadAssignments(): Promise<void> {
  const response = await invoke("get_all_assignments");
  const assignments = response as AssignmentItem[];

  const newAssignmentsMap = new Map();
  assignments.forEach((assignment) => {
    const key = `${assignment.group_id}-${assignment.day}-${assignment.module_index}`;
    newAssignmentsMap.set(key, {
      id: assignment.id,
      subject_name: assignment.subject_name,
      shorten: assignment.subject_shorten,
      color: assignment.subject_color,
      teacherId: assignment.teacher_id,
      subjectId: assignment.subject_id,
      classroomId: assignment.classroom_id,
      day: assignment.day,
      moduleIndex: assignment.module_index,
      groupId: assignment.group_id
    });
  });

  assignmentsStore.set(newAssignmentsMap);
}

// Manera eficiente de conseguir las asignaciones sin llamar a la base de datos
export function getLocalAssignment(groupId: number, day: string, moduleIndex: number) {
  const key = `${groupId}-${day}-${moduleIndex}`;
  let assignment;
  assignmentsStore.subscribe((map: any) => {
    assignment = map.get(key);
  })();
  return assignment;
}

// Funcion para cuando se suelta una materia en el modulo
export function handleAssignDrop(e: DragEvent | any, groupId: number, day: string, moduleIndex: number) {
  e.preventDefault?.(); // Make preventDefault optional

  // Handle both regular drag events and our custom events
  let data;

  if (e.dataTransfer) {
    // Regular drag event
    try {
      const jsonData = e.dataTransfer.getData("application/json");
      data = JSON.parse(jsonData);
    } catch (error) {
      console.error("Error parsing drag data:", error);
      return;
    }

    // Clean up UI
    const target = e.target as HTMLElement;
    if (target && target.classList) {
      target.classList.remove("drag-over");
    }
  } else {
    // Our custom event
    data = e.subject || e.data;
  }

  // Continue with the assignment logic
  if (data) {
    saveAssignment(groupId, day, moduleIndex, data.id, data.teacherId);
  }
}

export async function handleAssignClick(
  e: MouseEvent,
  assign_id: unknown
): Promise<void> {
  if (e.button === 1) { // Click a la rueda del mouse
    e.preventDefault;
    try {
      await invoke("delete_assignment", { assign_id });
      loadAssignments();
    } catch (e) {
      console.log(e);
    }
  }
}

export async function deleteAssignment(assign_id: unknown): Promise<void> {
  try {
    await invoke("delete_assignment", { assign_id })
    await loadAssignments()
    console.log("Deleted assignment with id:", assign_id);
  } catch (e) {
    console.log(e)
  }
}

export async function saveAssignment(
  groupId: number,
  day: string,
  moduleIndex: number,
  subjectId: number,
  teacherId: number
): Promise<void> {
  try {

    const moduleAvailable = await canAssignToModule(groupId, day, moduleIndex);
    if (!moduleAvailable) {
      addNotification({
        message: "El módulo ya está ocupado por otra materia",
        type: "error",
        timeout: 1000
      });
      return;
    }

    const teacherAvailable = await isTeacherAvailable(teacherId, day, moduleIndex);
    if (!teacherAvailable) {
      addNotification({
        message: "Profesor tiene este modulo del dia ocupado",
        type: "error",
        timeout: 1500
      });
      return;
    }
    await invoke("save_assignment", {
      group_id: groupId,
      day,
      module_index: moduleIndex,
      subject_id: subjectId,
      teacher_id: teacherId
    });

    // Update local store
    const key = `${groupId}-${day}-${moduleIndex}`;
    assignmentsStore.update((currentMap) => {
      const newMap = new Map(currentMap);
      newMap.set(key, {
        id: null, // This will be set by the database
        shorten: "", // We don't have this info here
        color: "", // We don't have this info here
        teacherId: teacherId,
        subjectId: subjectId,
      });
      return newMap;
    });

    // Reload assignments to get the full data with subject details
    await loadAssignments();
  } catch (error) {
    console.error("Failed to save assignment:", error);
  }
}


export const teacherHoursStore = derived(assignmentsStore, ($assignmentsStore) => {
  const hoursMap: Record<number, number> = {};

  for (const assignment of $assignmentsStore.values()) {
    if (!hoursMap[assignment.teacherId]) {
      hoursMap[assignment.teacherId] = 1;
    } else {
      hoursMap[assignment.teacherId] += 1;
    }
  }

  return hoursMap;
});

// Funcion para checar si el profesor no tiene el modulo ocupado (el mismo dia)
export async function isTeacherAvailable(
  teacherId: number,
  day: string,
  moduleIndex: number
): Promise<boolean> {
  try {
    const response = await invoke("check_teacher_availability", {
      teacherId,
      day,
      moduleIndex,
    });
    return response as boolean;
  } catch (error) {
    console.error("Error comprobando disponibilidad del profesor:", error);
    return false;
  }
}

/// Funcion que impide sustituir una materia dentro de un modulo
export async function canAssignToModule(
  groupId: number,
  day: string,
  moduleIndex: number
): Promise<boolean> {
  const assignment = getLocalAssignment(groupId, day, moduleIndex);
  return !assignment; // Devuelve true si el módulo está vacío
}

// Funcion para agregar un aula a un modulo/grupo/profesor
export async function assignClassroom(
  assignmentId: number,
  classroomId: number
): Promise<void> {
  try {
    await invoke("assign_classroom_to_assignment", {
      assignmentId,
      classroomId
    });
    await loadAssignments();
    addNotification({
      message: "Aula asignada correctamente",
      type: "success",
      timeout: 1000
    });
  } catch (error) {
    addNotification({
      message: `Error al asignar aula: ${error}`,
      type: "error",
      timeout: 3000
    });
  }
}

/// Funcion para verificar la disponibilidad de un aula
export async function checkClassroomAvailability(
  classroomId: number,
  day: string,
  moduleIndex: number
): Promise<boolean> {
  try {
    return await invoke("check_classroom_availability", {
      classroomId,
      day,
      moduleIndex
    });
  } catch (error) {
    console.error("Error checking classroom:", error);
    return false;
  }
}
