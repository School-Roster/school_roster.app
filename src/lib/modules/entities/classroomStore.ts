import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { addNotification } from "$lib/stores/notificationsStore";
import { loadAssignments } from "./assignments";

/**
  * Interfaz para los datos de los grupos
  * @property {number} id - Identificador único
  * @property {string} building_id - Identificador del edificio
  * @property {number} building_number - Numero del aula
  * @property {string} building_type - Tipo de aula (ej: Laboratorio) (si aplica)
  * @property {number} capacity - Capacidad del aula (si aplica)
  * @property {Array<[string, number]>} available - Disponibilidad del aula
  */
export interface ClassroomItem {
  id?: number;
  building_id: string,
  building_number: number | null,
  building_type: string,
  capacity: number | null,
  available?: Array<[string, number]>, // Opcional porque no se registra
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
  * @param {ClassroomItem} classroom: Clase del aula
  */
export async function addClassroom(classroom: ClassroomItem): Promise<void> {
  if (!classroom.building_number || !classroom.building_id) {
    alert("Por favor, rellene todos los campos");
    return;
  }

  await invoke("create_classroom", { cr: classroom });
  await loadClassrooms(); // Recarga las vistas
  await emit("classrooms_updated"); // Emite un evento para actualizar la vista de materias
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
  await invoke("update_classroom", { classroom: item });
  await loadClassrooms();
  await emit("classrooms_updated");
}

/**
  * Funcion para importar varios grupos, se utiliza en ImportExcel
  * @param {Record} headerMappings
  * @param {Array} data
  */
export async function importClassroomsFromXlsx(
  headerMappings: Record<string, string>,
  data: Array<Record<string, any>>
): Promise<void> {
  console.log("Raw data for classrooms:", data);

  // Prepare the classrooms to be imported
  const classroomsToImport = data.map((row) => {
    return {
      id: null,
      building_id: String(row.building_id || ''),
      building_number: row.building_number ? Number(row.building_number) : null,
      building_type: String(row.building_type || ''),
      capacity: row.capacity ? Number(row.capacity) : null
    };
  });

  if (classroomsToImport.length === 0) {
    throw new Error('No hay aulas válidas en el intento de importar datos');
  }

  try {
    await invoke("create_classrooms", { classroom: classroomsToImport });
    await loadClassrooms();
    await emit("classrooms_updated");
  } catch (error) {
    console.error('Hubo un error importando las aulas:', error);
    throw error;
  }
}
