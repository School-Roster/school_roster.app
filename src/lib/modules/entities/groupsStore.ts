import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";

/**
  * Interfaz para los datos de los grupos
  * @property {number} id - Identificador único
  * @property {number} grade - Numero del grado
  * @property {string} group - Grupo (A,B,C,...)
  * @property {string} career - Carrera (si aplica)
  * @property {number} students - Numero de alumnos (si aplica)
  */
export interface GroupItem {
  id: number;
  grade: number;
  group: string,
  career: string,
  students: number
}

/**
 * Lista todos los grupos registrados
 */
export const groups = writable<GroupItem[]>([]);

/**
 * Carga los grupos desde la base de datos
 */
export async function loadGroups() {
  const response = await invoke("get_groups");
  groups.set(response as GroupItem[]);
}
