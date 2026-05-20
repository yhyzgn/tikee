import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
  },
  server: {
    proxy: {
      '/api': 'http://0.0.0.0:9090',
      '/api-docs': 'http://0.0.0.0:9090',
    },
  },
});
