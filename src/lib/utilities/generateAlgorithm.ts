import { loadAssignments } from "$lib/modules/entities/assignments";
import { invoke } from "@tauri-apps/api"; 

export async function generateSchedule(): Promise<void> {
  await invoke("generate_schedule");
  await loadAssignments();
}
