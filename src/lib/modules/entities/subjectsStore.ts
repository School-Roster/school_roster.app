import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";

/**
  * Interfaz para los datos de las materias
  * @property {number} id - Identificador único
  * @property {string} name - Nombre de la materia
  * @property {string} shorten - Nombre corto
  * @property {string} color - Color de la materia
  * @property {string} spec - Especialidad a la que pertenece
  * @property {TeacherItem} assigned_teacher - Profesor asign
  */
export interface SubjectItem {
  id: number;
  name: string;
  shorten: string;
  color: string;
  spec: string;
}

/**
 * Lista todas las materias registradas
 */
export const subjects = writable<SubjectItem[]>([]);

/**
 * Carga las materias desde la base de datos
 */
export async function loadSubjects() {
  const response = await invoke("get_subjects");
  subjects.set(response as SubjectItem[]);
}
