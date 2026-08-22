import '@testing-library/jest-dom'
import { vi } from 'vitest'

vi.mock('@vueuse/core', async (importOriginal) => {
  const actual = await importOriginal()
  return {
    ...actual,
    useDark: vi.fn(() => ({ value: false })),
    useToggle: vi.fn(() => () => {
      // Mock 实现
    }),
    reactiveOmit: vi.fn((obj: any, key: string) => {
      const { [key]: _, ...rest } = obj
      return rest
    }),
  }
})
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: vi.fn(() => ({
    show: vi.fn(),
    hide: vi.fn(),
    minimize: vi.fn(),
    unminimize: vi.fn(),
    setFocus: vi.fn(),
    close: vi.fn(),
    toggleMaximize: vi.fn(),
    setAlwaysOnTop: vi.fn(),
    startDragging: vi.fn(),
  })),
}))

vi.mock('@tauri-apps/api/tray', () => ({
  TrayIcon: {
    new: vi.fn(),
    getById: vi.fn(),
  },
}))

vi.mock('@tauri-apps/api/menu', () => ({
  Menu: {
    new: vi.fn(),
  },
  MenuItem: {
    new: vi.fn(),
  },
}))

vi.mock('@tauri-apps/plugin-dialog', () => ({
  ask: vi.fn(),
}))

vi.mock('@tauri-apps/plugin-fs', () => ({
  stat: vi.fn(),
}))

vi.mock('@tauri-apps/plugin-log', () => ({
  debug: vi.fn(),
  info: vi.fn(),
  warn: vi.fn(),
  error: vi.fn(),
}))
