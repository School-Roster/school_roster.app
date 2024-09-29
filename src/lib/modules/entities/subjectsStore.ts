import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { type SimpleTeacherItem } from "./teachersStore";

/**
  * Interfaz para los datos de las materias
  * @property {number} id - Identificador único
  * @property {string} name - Nombre de la materia
  * @property {string} shorten - Nombre corto
  * @property {string} color - Color de la materia
  * @property {string} spec - Especialidad a la que pertenece
  * @property {SimpleTeacherItem} assigned_teacher - Profesor asign
  */
export interface SubjectItem {
  id: number;
  name: string;
  shorten: string;
  color: string;
  spec: string;
  assigned_teacher: SimpleTeacherItem | null;
}

/**
 * Lista todas las materias registradas
 */
export const subjects = writable<SubjectItem[]>([]);

/**
 * Lista de todas las materias con profesores asignados
 */
export const subjectsWithTeachers = writable<SubjectItem[]>([]);

/**
 * Carga las materias desde la base de datos
 */
export async function loadSubjects() {
  const response = await invoke("get_subjects");
  subjects.set(response as SubjectItem[]);
}

/**
 * Carga las materias con los profesores asignados desde la base de datos
 (esto ahorra mas recursos que haciendolo desde ts)
 */
export async function loadSubjectsWithTeachers() {
  const response = await invoke("get_subjects_with_teachers");
  console.log("Response from rust: ", response);
  subjectsWithTeachers.set(response as SubjectItem[]);
}
