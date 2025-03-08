import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  optimizeDeps: {
    exclude: ['lucide-react'],
  },
  base: '/app/',
  build: {
    outDir: 'build',
    emptyOutDir: true
  },
  server: {
    proxy: {
      '/app': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/app/, ''),
      },
      '/chat': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
});
