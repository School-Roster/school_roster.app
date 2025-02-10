import { invoke } from "@tauri-apps/api";
import { writable, type Writable } from "svelte/store";

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
      shorten: assignment.subject_shorten,
      color: assignment.subject_color,
      teacherId: assignment.teacher_id,
      subjectId: assignment.subject_id,
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
export async function handleAssignDrop(
  e: DragEvent,
  groupId: number,
  day: string,
  moduleIndex: number,
): Promise<void> {
  e.preventDefault();
  const target = e.target as HTMLElement;
  target.classList.remove("drag-over");

  // Para evitar llamadas a la base de datos actualizamos local
  const subjectData: string | undefined =
    e.dataTransfer?.getData("application/json");
  if (!subjectData) return;

  const subject: any = JSON.parse(subjectData);
  // Creamos una llave unica para el assign
  const key = `${groupId}-${day}-${moduleIndex}`;

  try {
    await invoke("save_assignment", {
      group_id: groupId,
      day,
      module_index: moduleIndex,
      subject_id: subject.id,
      teacher_id: subject.teacherId,
    });
    assignmentsStore.update((currentMap) => {
      const newMap = new Map(currentMap);
      newMap.set(key, subject);
      return newMap;
    });
  } catch (error) {
    console.error("Failed to save assignment:", error);
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

