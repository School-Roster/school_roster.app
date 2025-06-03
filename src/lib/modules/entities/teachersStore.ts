import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { subjects, type SubjectItem } from "./subjectsStore";
import { emit } from "@tauri-apps/api/event";

/**
  * Interfaz para los datos de los profesores
  * @property {number} id - Identificador único
  * @property {string} name - Nombre del profesor
  * @property {string} father_lastname - Apellido paterno
  * @property {string} mother_lastname - Apellido materno (opcional)
  * @property {string} email - Correo electrónico (opcional)
  * @property {string} phone - Número de teléfono (opcional)
  * @property {string} degree - Grado académico (opcional)
  * @property {number} commisioned_hours - Horas comisionadas (opcional)
  * @property {number} active_hours - Horas activas (opcional)
  * @property {string[]} preferred_days - Dias preferidos del profesor (opcional)
  * @property {number[]} preferred_modules - Modulos preferidos del profesor (opcional)
  */
export interface TeacherItem {
  id?: number;
  name: string;
  father_lastname: string;
  mother_lastname: string;
  email: string;
  phone: string;
  degree: string;
  commisioned_hours: number;
  active_hours: number;
  performance: number;
  preferred_days?: string[];
  preferred_modules?: number[];
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
 * Manda la nueva materia a la base de datos en rust
 */
export async function addTeacher(teacher: TeacherItem, selectedSubjects: SubjectItem[]): Promise<void> {
  if (!teacher.name || !teacher.father_lastname) {
    alert("Por favor, rellene todos los campos necesarios");
    return;
  }

  if (teacher.commisioned_hours < 0 || teacher.performance < 0) {
    alert("Por favor, rellene los campos con valores positivos");
    return;
  }

  if (selectedSubjects.length > 0) {
    emit("subjects_with_teachers_updated");
  }

  // Registrar nuevo profesor
  await invoke("add_teacher", {
    teacher,
    subjects:
      selectedSubjects.length > 0 ? selectedSubjects.map((s) => s.id) : null, // Pasamos solo los ids de las materias seleccionadas
  });
  await loadTeachers(); // Recarga las materias
  await emit("teachers_updated"); // Emite un evento para actualizar la vista de materias
}

export async function editTeacher(teacher: TeacherItem, selectedSubjects: SubjectItem[]): Promise<void> {
  if (!teacher.name || !teacher.father_lastname) {
    alert("Por favor, rellene todos los campos necesarios");
    return;
  }

  if (teacher.commisioned_hours < 0 || teacher.performance < 0) {
    alert("Por favor, rellene los campos numericos con valores positivos");
    return;
  }

  if (selectedSubjects.length > 0) {
    emit("subjects_with_teachers_updated");
  }

  // Registrar nuevo profesor
  await invoke("edit_teacher", {
    teacher,
    subjects:
      selectedSubjects.length > 0 ? selectedSubjects.map((s) => s.id) : null, // Pasamos solo los ids de las materias seleccionadas
  });
  await loadTeachers(); // Recarga las materias
  await emit("teachers_updated"); // Emite un evento para actualizar la vista de materias
}

/**
 * Carga a los profesores de la base de datos
 */
export async function loadTeachers(): Promise<void> {
  const response: [TeacherItem, number[]][] = await invoke<[TeacherItem, number[]][]>('get_all_teachers'); // Tuple para obtener los profesores y las materias asignadas

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
}

/**
  * Funcion para importar varios grupos, se utiliza en ImportExcel
  * @param {Record} headerMappings
  * @param {Array} data
  */
export async function importTeachersFromXlsx(
  headerMappings: Record<string, string>,
  data: Array<Record<string, any>>
): Promise<void> {
  // Prepare the subjects to be imported
  const teacherToImport = data.map((row) => {
    return {
      id: null,
      name: String(row.name || ''),
      father_lastname: String(row.father_lastname || ''),
      mother_lastname: String(row.mother_lastname || ''),
      email: String(row.email || ''),
      phone: String(row.phone || ''),
      degree: String(row.degree || ''),
      commisioned_hours: Number(row.commisioned_hours || 0),
      active_hours: Number(row.active_hours || 0),
      performance: Number(row.performance || 0),
      preferred_days: row.preferred_days ? JSON.parse(row.preferred_days) : [],
      preferred_modules: row.preferred_modules ? JSON.parse(row.preferred_modules) : []
    };
  });

  if (teacherToImport.length === 0) {
    throw new Error('No hay grupos validos en el intento de importar datos');
  }

  try {
    await invoke("create_teachers", { teacher: teacherToImport });
    await loadTeachers();
    await emit("teachers_updated");
  } catch (error) {
    console.error('Hubo un error importando las materias:', error);
    throw error;
  }
}
