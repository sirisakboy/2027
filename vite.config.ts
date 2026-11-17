import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte()],
  server: { 
    port: 1420, 
    strictPort: true,
    host: '0.0.0.0',
    cors: true,
    hmr: { host: 'localhost', port: 1421 },
    // @ts-ignore
    allowedHosts: true
  },
  preview: {
    host: '0.0.0.0',
    port: 1420
  },
  clearScreen: false,
})
