import { invoke } from "@tauri-apps/api";
import { writable, type Writable } from "svelte/store";

/**
  * Interfaz para los datos de los grupos
  * @property {number} group_id - Identificador del grupo
  * @property {string} day - Dia asignado
  * @property {number} module_index - Espacio (modulo)
  * @property {number} subject_id - Id de la materia
  */
export interface AssignmentItem{
  group_id: number,
  day: string,
  module_index: number,
  subject_id: number,
}

export const assigns = writable<AssignmentItem[]>([]);

export async function loadAssignments(): Promise<void> {
  const response = await invoke("get_all_assignments");
  console.log(response);
  assigns.set(response as AssignmentItem[]);
}
