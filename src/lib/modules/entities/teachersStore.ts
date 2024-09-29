import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { subjects, type SubjectItem } from "./subjectsStore";

/**
  * Interfaz para los datos de los profesores
  * @property {number} id - Identificador único
  * @property {string} name - Nombre del profesor
  * @property {string} father_lastname - Apellido paterno
  * @property {string} mother_lastname - Apellido materno (opcional)
  * @property {string} email - Correo electrónico (opcional)
  * @property {string} phone - Número de teléfono (opcional)
  * @property {string} degree - Grado académico (opcional)
  * @property {number} commissioned_hours - Horas comisionadas (opcional)
  * @property {number} active_hours - Horas activas (opcional)
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
  * Interfaz para los datos de los profesores (simple)
  * @property {number} id - Identificador único
  * @property {string} name - Nombre del profesor
  * @property {string} father_lastname - Apellido paterno
  */
export interface SimpleTeacherItem {
  id: number;
  name: string;
  father_lastname: string;
}


/**
 * Lista todos los profesores registrados
 */
export const teachers = writable<TeacherItem[]>([]);

/**
 * Carga a los profesores de la base de datos
 */
export async function loadTeachers() {
  const response = await invoke<[TeacherItem, number[]][]>('get_all_teachers'); // Tuple para obtener los profesores y las materias asignadas

  let subjectList: SubjectItem[] = [];
  // Necesitamos la lista de materias para poder asignarlas a los profesores sin hacer otra petición
  subjects.subscribe((value: SubjectItem[]) => {
    subjectList = value;
  })();

  const teachersArray = response.map(([teacher, subjectId]) => ({
    ...teacher,
    assigned_subjects: subjectId.map(id => {
      // Aprovechamos la lista de materias para obtener el nombre de la materia sin necesidad de hacer otra petición
      const subject = subjectList.find(subject => subject.id === id);
      return subject ? subject.name : '';
    })
  }));

  teachers.set(teachersArray as TeacherItem[]);
  console.log(teachersArray);
}
