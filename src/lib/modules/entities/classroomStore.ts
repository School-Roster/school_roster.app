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
