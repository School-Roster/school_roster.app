import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [svelte()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/core/**"],
    },
  },
  resolve: {
    alias: {
      // Aquí configuramos alias para resolver módulos npm
      // Puedes agregar tantos como necesites
      // En este caso, estamos configurando alias para resolver '@lukeed/uuid'
      // Esto significa que podrás importar '@lukeed/uuid' sin tener que especificar la ruta completa
      // Si tuvieras más paquetes npm para resolver, agregarías más entradas a este objeto
      "@lukeed/uuid": "@lukeed/uuid",
    },
  },
}));
