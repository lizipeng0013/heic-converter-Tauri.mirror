import { defineConfig } from 'vitest/config'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: 'jsdom',
    globals: true,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['src/**/*.ts', 'src/**/*.vue'],
      exclude: ['src/vite-env.d.ts', 'src/**/types/**'],
    },
    include: ['src/__tests__/**/*.test.ts'],
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
    setupFiles: ['./src/__tests__/setup.ts'],
  },
})
