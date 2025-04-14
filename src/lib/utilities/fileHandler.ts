import { invoke } from "@tauri-apps/api"; 

export async function exportFile(): Promise<void> {
  await invoke("export_file");
}

export async function importFile(): Promise<void> {
  await invoke("import_file");
}
