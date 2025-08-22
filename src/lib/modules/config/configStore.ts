import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

interface Config {
  days: string[];
  modulesPerDay: number;
  moduleDuration: number;
  durationUnit: 'minutes' | 'hours' | string;
  hasBreaks: boolean;
  breakCount: number;
  breakDuration: number;
  breakPositions: number[];
}

interface SchoolInfo {
  name: string;
  logo_path: string | null;
}

const defaultConfig: Config = {
  days: ['Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes'],
  modulesPerDay: 6,
  moduleDuration: 60,
  durationUnit: 'minutes',
  hasBreaks: false,
  breakCount: 1,
  breakDuration: 30,
  breakPositions: [2]
};

export const configStore = writable<Config>(defaultConfig);

export async function loadConfig() {
  try {
    const config = await invoke<Config>('get_config');
    if (config) {
      configStore.set(config);
    }
  } catch (error) {
    console.error('Error loading config:', error);
  }
}

export async function saveConfig(config: Config) {
  try {
    await invoke('save_config', { config });
    configStore.set(config);
  } catch (error) {
    console.error('Error saving config:', error);
  }
} 


export async function loadSchoolInfo() {
  try {
    const info = await invoke<SchoolInfo>('get_school_info');
    console.log("School data:", info);
    schoolStore.set(info);
  } catch (error) {
    console.error('Error loading school info:', error);
  }
}


export async function saveSchoolInfo(name: string, logoPath: string) {
  try {
    console.log("Sent path: ", logoPath);
    await invoke('save_school_info', { name, logoPath });
    //@ts-ignore
    schoolStore.set({ name, logo_path: logoPath });
  } catch (error) {
    console.error('Error saving school info:', error);
  }
}

export async function selectSchoolLogo(): Promise<string | null> {
  try {
    const path = await invoke<string>('select_school_logo');
    return path;
  } catch (error) {
    console.error('Error selecting logo:', error);
    return null;
  }
}

export const schoolStore = writable<SchoolInfo>({ name: '', logo_path: null })
