import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import type { SubjectItem } from "./subjectsStore";

/**
  * Interfaz para los datos de los grupos
  * @property {number} id - Identificador único
  * @property {number} grade - Numero del grado
  * @property {string} group - Grupo (A,B,C,...)
  * @property {string} career - Carrera (si aplica)
  * @property {number} students - Numero de alumnos (si aplica)
  * @property {number} max_modules_per_day - Modulos al dia (si aplica)
  * @property {SubjectItem} required_subjects - Materias asignadas (si aplica)
  */
export interface GroupItem {
  id?: number,
  grade: number | null ,
  group: string,
  career: string,
  students: number | null,
  max_modules_per_day?: number | null,
  required_subjects?: SubjectItem[]
}

/**
 * Lista todos los grupos registrados
 */
export const groups = writable<GroupItem[]>([]);

/**
 * Carga los grupos desde la base de datos
 */
export async function loadGroups(): Promise<void> {
  // Fetch groups with their required subjects
  const response: [GroupItem, SubjectItem[]][] = await invoke<[GroupItem, SubjectItem[]][]>('get_groups');

  // Format the response to match the GroupItem interface
  const formattedGroups: GroupItem[] = response.map(([group, subjects]) => ({
    ...group,
    required_subjects: subjects, // Assign subjects to the group
  }));

  // Update the groups store
  groups.set(formattedGroups);
}

/**
  * Funcion para agregar un nuevo grupo a la base de datos
  * @param {GroupItem} group
  * @param {SubjectItem} subjects
  */
export async function addGroup(
  group: GroupItem,
  subjects: SubjectItem[]
): Promise<void> {
  if (!group.grade || !group) {
    alert("Por favor, rellene todos los campos");
    return;
  }

  await invoke("create_group", {
    g: {
      grade: group.grade,
      group: group.group,
      career: group.career,
      students: group.students,
      max_modules_per_day: group.max_modules_per_day,
    },
    subjects:
      subjects.length > 0 ? subjects.map((s) => s) : null,
  });
  await loadGroups(); // Recarga las vistas
  await emit("groups_updated"); // Emite un evento para actualizar la vista de materias
}

/**
  * Funcion para editar un grupo existente
  * @param {GroupItem} item
  * @param {SubjectItem} subjects
  */
export async function editGroup(item: GroupItem, subjects: SubjectItem[]): Promise<void> {
  if (!item) return;
  if (!item.grade || !item.group) {
    alert("Por favor, rellene todos los campos");
    return;
  }
  // TODO: Pasar el item directamente en vez de sus propiedades (mas limpio)
  await invoke("update_group", {
    g: item,
    subjects:
      subjects.length > 0 ? subjects.map((s) => s) : null,
  });
  await loadGroups();
  await emit("groups_updated");
}

/**
  * Funcion para importar varios grupos, se utiliza en ImportExcel
  * @param {Record<string, string>} headerMappings
  * @param {Array<Record<string, any>>} data
  */
export async function importGroupsFromXlsx(
  headerMappings: Record<string, string>,
  data: Array<Record<string, any>>
): Promise<void> {
  console.log("Raw data for groups:", data);

  // Prepare the groups to be imported
  const groupsToImport = data.map((row) => {
    return {
      id: null,
      grade: row.grade ? Number(row.grade) : null,
      group: String(row.group || ''),
      career: String(row.career || ''),
      students: row.students ? Number(row.students) : null,
      max_modules_per_day: row.max_modules_per_day ? Number(row.max_modules_per_day) : null,
    };
  });

  if (groupsToImport.length === 0) {
    throw new Error('No hay grupos válidos en el intento de importar datos');
  }

  try {
    await invoke("create_groups", { groups: groupsToImport });
    await loadGroups();
    await emit("groups_updated");
  } catch (error) {
    console.error('Hubo un error importando los grupos:', error);
    throw error;
  }
}
