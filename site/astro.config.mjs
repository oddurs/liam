import { defineConfig } from 'astro/config';

export default defineConfig({
  // A fixed port that nothing else on the machine claims by default.
  server: { port: 4417 },
  build: {
    // One HTML response carries its CSS, so first paint needs no second request.
    inlineStylesheets: 'always',
  },
});
