import { invoke } from "@tauri-apps/api"; 

export async function exportFile(): Promise<void> {
  await invoke("export_database");
}
