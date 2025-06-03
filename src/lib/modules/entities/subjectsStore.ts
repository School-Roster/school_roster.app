import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
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
  id?: number;
  name: string;
  shorten: string;
  color: string;
  spec: string;
  required_modules?: number | null;
  priority?: number | null;
  assigned_teacher?: SimpleTeacherItem | null;
}

/**
 * Lista todas las materias registradas
 */
export const subjects = writable<SubjectItem[]>([]);

/**
  * Variable que expone una materia seleccionada (en grid)
  */
export const selectedSubject = writable<SubjectItem | null>(null);

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

export async function editSubject(item: SubjectItem): Promise<void> {
  if (!item) return;
  if (!item.name || !item.shorten) {
    alert("Por favor, rellene todos los campos");
    return;
  }
  // TODO: Pasar el item directamente en vez de sus propiedades (mas limpio)
  await invoke("update_subject", { subject: item });
  await loadSubjects();
  await emit("subjects_updated");
}

// Manda la nueva materia a la base de datos en rust
export async function addSubject(subject: SubjectItem): Promise<void> {
  if (!subject.name) {
    alert("Por favor, rellene todos los campos");
    return;
  }
  if (!subject.shorten) {
    // Crea una abreviacion si no se proporciono alguna
    subject.shorten = subject.name.substring(0, 3).toUpperCase();
  }

  if (subject.required_modules == null) {
    subject.required_modules = 0;
  }

  await invoke("create_subject", { subject });
  await loadSubjects(); // Recarga las materias
  await emit("subjects_updated"); // Emite un evento para actualizar la vista de materias
}

/**
 * Funcion para importar varios grupos, se utiliza en ImportExcel
 * @param {Record} headerMappings
 * @param {Array} data
 */
export async function importSubjectsFromXlsx(
  headerMappings: Record<string, string>,
  data: Array<Record<string, any>>
): Promise<void> {
  console.log("Raw data:", data);

  // Prepare the subjects to be imported
  const subjectToImport = data
    .map((row) => {
      return {
        id: null,
        name: String(row.name || ""),
        shorten: row.shorten ? String(row.shorten || "") : "",
        color: row.color ? String(row.color || "") : null,
        spec: row.spec ? String(row.spec || "") : null,
        required_modules: row.required_modules
          ? Number(row.required_modules || 0)
          : 0,
        priority: row.priority ? Number(row.priority || 0) : 0,
      };
    })
    .filter((subject) => subject.name);

  if (subjectToImport.length === 0) {
    throw new Error("No hay grupos validos en el intento de importar datos");
  }

  try {
    await invoke("create_subjects", { subject: subjectToImport });
    await loadSubjects();
    await emit("subjects_updated");
  } catch (error) {
    console.error("Hubo un error importando las materias:", error);
    throw error;
  }
}
