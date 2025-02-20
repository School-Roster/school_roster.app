import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";

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

/**
  * Funcion para agregar un nuevo grupo a la base de datos
  * @param {string} building_id
  * @param {number} building_number
  * @param {string} building_type
  * @param {number} capacity
  */
export async function addClassroom(building_id: string, building_number: number, building_type: string, capacity: number): Promise<void> {
  if (!building_number || !building_id) {
    alert("Por favor, rellene todos los campos");
    return;
  }

  await invoke("create_classroom", {
    building_id,
    building_number,
    building_type: building_type || null,
    capacity: capacity || null,
  });
  await loadClassrooms(); // Recarga las vistas
  await emit("classrooms_updated"); // Emite un evento para actualizar la vista de materias

  // Limpiamos los campos
  building_number = 0;
  building_id = "";
  building_type = "";
  capacity = 0;
}

/**
  * Funcion para agregar un nuevo grupo a la base de datos
  * @param {ClassroomItem} item
  */
export async function editClassroom(item: ClassroomItem): Promise<void> {
  if (!item) return;
  if (!item.building_id || !item.building_number) {
    alert("Por favor, rellene todos los campos");
    return;
  }
  // TODO: Pasar el item directamente en vez de sus propiedades (mas limpio)
  await invoke("update_classroom", {
    id: item.id,
    building_id: item.building_id,
    building_number: item.building_number,
    building_type: item.building_type,
    capacity: item.capacity,
  });
  await loadClassrooms();
  await emit("classrooms_updated");
}

/**
  * Funcion para importar varios grupos, se utiliza en ImportExcel
  * @param {Record} headerMappings
  * @param {Array} excelData
  */
export async function importClassroomsFromXlsx(
  headerMappings: Record<string, string>,
  excelData: Array<Record<string, unknown>>
): Promise<void> {
  console.log("Raw data:", excelData);

  // Checar por campos requeridos no importados
  const required: string[] = ['building_id', 'building_number'];
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
  console.log("columnMap: ", columnMap);

  // Preparar los grupos que seran importados
  const classroomToImport = excelData
    .slice(columnMap.building_id.startRow, columnMap.building_id.endRow || undefined)
    .map(row => {
      console.log(String(row['Edificio']));
      return {
        id: null,
        building_id: String(row['Edificio']),
        building_number: Number(row['Numero de aula']),
        building_type: columnMap.building_number
          ? String(row['Tipo'] || '')
          : null,
        capacity: columnMap.capacity
          ? Number(row['Capacidad'] || '')
          : null
      };
    })
    .filter(classroom => classroom.building_id && classroom.building_number);

  if (classroomToImport.length === 0) {
    throw new Error('No hay grupos validos en el intento de importar datos');
  }

  try {
    await invoke("create_classrooms", { classroom: classroomToImport });
    await loadClassrooms();
    await emit("classrooms_updated");
  } catch (error) {
    console.error('Hubo un error importando las aulas:', error);
    throw error;
  }
}
