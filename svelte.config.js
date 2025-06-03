import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      // fallback: '200.html' // mantener el fallback para SPA routing
      pages: 'build',
      assets: 'build',
      fallback: 'index.html', // Changed from 200.html to index.html
      precompress: false,
      strict: false // This is important for dynamic routes
    }),
    prerender: {
      handleHttpError: ({ path, referrer, message }) => {
        // Ignore errors for dynamic routes
        if (path.startsWith('/window/')) {
          return;
        }
        throw new Error(message);
      },
      entries: [
        '/',
        '/window/teacherSchedule',
        '/window/groupSchedule',
        '/window/subjectSchedule',
        '/window/subjects',
        '/window/teachers',
        '/window/login',
        '/window/classroom',
        '/window/groups',
        '/window/settings',
        '/window/ai'
      ]
    },
    alias: {
      $styles: "./src/styles",
    },
  },
};

export default config;
