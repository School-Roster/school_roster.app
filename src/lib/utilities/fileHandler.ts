import { loadAssignments } from "$lib/modules/entities/assignments";
import { loadClassrooms } from "$lib/modules/entities/classroomStore";
import { loadGroups } from "$lib/modules/entities/groupsStore";
import { loadSubjects, loadSubjectsWithTeachers } from "$lib/modules/entities/subjectsStore";
import { loadTeachers } from "$lib/modules/entities/teachersStore";
import { invoke } from "@tauri-apps/api"; 

export async function exportFile(): Promise<void> {
  await invoke("export_file");
}

export async function importFile(): Promise<void> {
  await invoke("import_file");
  await loadTeachers();
  await loadSubjects();
  await loadSubjectsWithTeachers();
  await loadGroups();
  await loadClassrooms();
  await loadAssignments();
}

export async function deleteAll(): Promise<void> {
  await invoke("delete_all_data");
  await loadTeachers();
  await loadSubjects();
  await loadSubjectsWithTeachers();
  await loadGroups();
  await loadClassrooms();
  await loadAssignments();
}
