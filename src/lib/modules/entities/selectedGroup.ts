//aqui podemos un id para las materias pre-asignadas y que puedan filtrarse
import { writable } from "svelte/store";

// Como group.id es un número, usamos number | null.
export const selectedGroup = writable<number | null>(null);
