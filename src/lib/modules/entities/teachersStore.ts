import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import type { SubjectItem } from "./subjectsStore";

/**
  * Interfaz para los datos de las materias
  * @property {number} id - Identificador único
  * @property {string} name - Nombre del profesor
  * @property {string} father_lastname - Apellido paterno
  * @property {string} mother_lastname - Apellido materno (opcional)
  * @property {string} email - Correo electrónico (opcional)
  * @property {string} phone - Número de teléfono (opcional)
  * @property {string} degree - Grado académico (opcional)
  * @property {number} commissioned_hours - Horas comisionadas (opcional)
  * @property {number} active_hours - Horas activas (opcional)
  * @property {SubjectItem[]} assigned_subjects - Materias asignadas
  */
export interface TeacherItem {
  id: number;
  name: string;
  father_lastname: string;
  mother_lastname: string;
  email: string;
  phone: string;
  degree: string;
  commissioned_hours: number;
  active_hours: number;
  performance: number;
}

/**
 * Lista todos los profesores registrados
 */
export const teachers = writable<TeacherItem[]>([]);

/**
 * Carga a los profesores de la base de datos
 */
export async function loadTeachers() {
  const response = await invoke("get_all_teachers");
  teachers.set(response as TeacherItem[]);
}
