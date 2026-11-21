import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  base: '/architecture/',
  build: {
    outDir: '../../docs/architecture',
    emptyOutDir: true,
  },
})