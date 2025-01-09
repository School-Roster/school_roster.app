import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";

/**
  * Interfaz para los datos de los grupos
  * @property {number} id - Identificador único
  * @property {string} building_id - Identificador del edificio
  * @property {number} building_number - Numero del aula
  * @property {string} building_type - Tipo de aula (ej: Laboratorio) (si aplica)
  * @property {number} capacity - Capacidad del aula (si aplica)
  */
export interface ClassroomItem {
  id: number;
  building_id: string,
  building_number: number;
  building_type: string,
  capacity: number
}

/**
 * Lista todos los grupos registrados
 */
export const classrooms = writable<ClassroomItem[]>([]);

/**
 * Carga los grupos desde la base de datos
 */
export async function loadClassrooms() {
  const response = await invoke("get_classrooms");
  classrooms.set(response as ClassroomItem[]);
}
