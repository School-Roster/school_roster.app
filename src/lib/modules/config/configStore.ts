import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

interface Config {
  days: string[];
  modulesPerDay: number;
  moduleDuration: number;
  durationUnit: 'minutes' | 'hours';
  hasBreaks: boolean;
  breakCount: number;
  breakDuration: number;
  breakPositions: number[];
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
