// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    // adapter: adapter(),
    adapter: adapter({
      fallback: '200.html' // this file will act as the catch-all fallback
    }),
    prerender: {
      entries: [] // or keep default: ['*'] if your static pages are prerenderable
    },
    alias: {
      $styles: "./src/styles",
    },
  },
};

export default config;
