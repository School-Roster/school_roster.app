import { invoke } from "@tauri-apps/api";
import { writable, type Writable } from "svelte/store";
import { subjects } from "./subjectsStore";

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

  console.log(assignments)

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
    await invoke("delete_assignment", {assign_id})
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

