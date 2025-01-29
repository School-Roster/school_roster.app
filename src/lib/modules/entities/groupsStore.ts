import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";

/**
  * Interfaz para los datos de los grupos
  * @property {number} id - Identificador único
  * @property {number} grade - Numero del grado
  * @property {string} group - Grupo (A,B,C,...)
  * @property {string} career - Carrera (si aplica)
  * @property {number} students - Numero de alumnos (si aplica)
  */
export interface GroupItem {
  id: number;
  grade: number;
  group: string,
  career: string,
  students: number
}

/**
 * Lista todos los grupos registrados
 */
export const groups = writable<GroupItem[]>([]);

/**
 * Carga los grupos desde la base de datos
 */
export async function loadGroups() {
  const response = await invoke("get_groups");
  groups.set(response as GroupItem[]);
}

/**
  * Funcion para agregar un nuevo grupo a la base de datos
  * @param {number} grade
  * @param {string} group
  * @param {string} career
  * @param {number} students
  */
export async function addGroup(
  grade: number, group: string, career: string | null, students: number | null
): Promise<void> {
  if (!grade || !group) {
    alert("Por favor, rellene todos los campos");
    return;
  }

  await invoke("create_group", {
    grade,
    group,
    career: career || null,
    students: students || null,
  });
  await loadGroups(); // Recarga las vistas
  await emit("groups_updated"); // Emite un evento para actualizar la vista de materias

  // Limpiamos los campos
  grade = 0;
  group = "";
  career = "";
  students = 0;
}

/**
  * Funcion para editar un grupo existente
  * @param {GroupItem} item
  */
export async function editGroup(item: GroupItem): Promise<void> {
  if (!item) return;
  if (!item.grade || !item.group) {
    alert("Por favor, rellene todos los campos");
    return;
  }
  // TODO: Pasar el item directamente en vez de sus propiedades (mas limpio)
  await invoke("update_group", {
    id: item.id,
    grade: item.grade,
    group: item.group,
    career: item.career,
    students: item.students,
  });
  await loadGroups();
  await emit("groups_updated");
}

/**
  * Funcion para importar varios grupos, se utiliza en ImportExcel
  * @param {Array} mappings
  * @param {Array} excelData
  */
export async function importGroupsFromXlsx(
  mappings: Array<{
    field: { key: string, name: string };
    range: { column: string, startRow: number, endRow: number | null };
  }>,
  excelData: Array<Record<string, unknown>>
): Promise<void> {
  console.log("Raw data:", excelData);

  // Checar por campos requeridos no importados
  const required: string[] = ['grade', 'group'];
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
  const groupsToImport = excelData
    .slice(columnMap.grade.startRow, columnMap.grade.endRow || undefined)
    .map(row => {
      const rowValues = Object.values(row);
      return {
        id: null,
        grade: Number(row['GRADO']),
        group: String(row['GRUPO']),
        career: columnMap.career
          ? String(row['CARRERA'] || '')
          : null,
        students: columnMap.students
          ? Number(row['Cantidad'] || '')
          : null
      };
    })
    .filter(group => group.grade && group.group);

  if (groupsToImport.length === 0) {
    throw new Error('No hay grupos validos en el intento de importar datos');
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
