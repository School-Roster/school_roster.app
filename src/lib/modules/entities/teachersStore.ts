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
export async function loadTeachers(): Promise<void> {
  const response: [TeacherItem, number[]][]  = await invoke<[TeacherItem, number[]][]>('get_all_teachers'); // Tuple para obtener los profesores y las materias asignadas

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
  * @param {Array} headerMappings
  * @param {Array} excelData
  */
export async function importTeachersFromXlsx(
  headerMappings: Record<string, string>,
  excelData: Array<Record<string, unknown>>
): Promise<void> {
  console.log("Raw data:", excelData);

  // Checar por campos requeridos no importados
  const required: string[] = ['name', 'father_lastname'];
  const missingFields: string[] = required.filter(
    field => !mappings.some(m => m.field.key === field)
  );
  if (missingFields.length > 0) {
    throw new Error(`Faltan campos necesarios: ${missingFields.join(',')}`);
  }

  // Convierte la columna en el index
  const columnLetterToIndex = (letter: string): number => {
    letter = letter.toUpperCase();
    return letter.split('').reduce((acc, char) =>
      acc * 26 + (char.charCodeAt(0) - 'A'.charCodeAt(0) + 1), 0) - 1;
  };

  // Crear diccionario del mapeo
  const columnMap = mappings.reduce((acc, mapping) => {
    if (mapping.range.column) {
      acc[mapping.field.key] = {
        columnIndex: columnLetterToIndex(mapping.range.column),
        startRow: mapping.range.startRow - 2,
        endRow: mapping.range.endRow ? mapping.range.endRow - 1 : undefined
      };
    }
    return acc;
  }, {} as Record<string, { columnIndex: number; startRow: number; endRow?: number }>);

  // Preparar los grupos que seran importados
  const teacherToImport = excelData
    .slice(columnMap.name.startRow, columnMap.name.endRow || undefined)
    .map(row => {
      console.log(row);
      return {
        id: null,
        name: String(row['Nombre']),
        father_lastname: String(row['Apellido paterno']),
        mother_lastname: columnMap.mother_lastname
          ? String(row['Apellido materno'] || '')
          : null,
        email: columnMap.email
          ? String(row['correo'] || '')
          : null,
        phone: columnMap.phone
          ? String(row['Telefono'] || '')
          : null,
        degree: columnMap.degree
          ? String(row['Titulo'] || '')
          : null,
        comissioned_hours: columnMap.comissioned_hours
          ? Number(row['Horas (comision)'] || '')
          : null,
        active_hours: columnMap.active_hours
          ? Number(row['Horas (activas)'] || '')
          : null,
        performance: columnMap.performance
          ? Number(row['Rendimiento'] || '')
          : null
      };
    })
    .filter(teacher => teacher.name&& teacher.father_lastname);

  if (teacherToImport.length === 0) {
    throw new Error('No hay profesores validos en el intento de importar datos');
  }

  try {
    await invoke("create_teachers", { teacher: teacherToImport });
    await loadTeachers();
    await emit("teachers_updated");
  } catch (error) {
    console.error('Hubo un error importando profesores:', error);
    throw error;
  }
}
