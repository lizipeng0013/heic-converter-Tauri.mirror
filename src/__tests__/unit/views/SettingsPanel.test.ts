import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import SettingsPanel from '@/views/settings/SettingsPanel.vue'
import { useConversionStore } from '@/stores/conversionStore'
import { createPinia, setActivePinia } from 'pinia'
import { open as openDialog } from '@tauri-apps/plugin-dialog'

vi.mock('@/stores/conversionStore', () => ({
  useConversionStore: vi.fn(),
}))

vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}))

vi.mock('@tauri-apps/api/path', () => ({
  downloadDir: vi.fn(),
}))

describe('SettingsPanel.vue', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('renders settings panel', async () => {
    const mockStore = {
      outputFolder: '/test/output',
      settings: { format: 'jpeg', quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    }
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any)
    vi.mocked(openDialog).mockResolvedValue('/new/output')

    const wrapper = mount(SettingsPanel, {
      global: {
        stubs: {
          Settings2: true,
          FolderOpen: true,
          ChevronDown: true,
          Square: true,
          Button: {
            template: '<button><slot /></button>',
          },
          Slider: {
            template: '<div class="slider"><slot /></div>',
          },
          Tooltip: {
            template: '<div><slot /></div>',
          },
          TooltipContent: {
            template: '<div><slot /></div>',
          },
          TooltipTrigger: {
            template: '<div><slot /></div>',
          },
        },
      },
    })

    expect(wrapper.find('.bg-card').exists()).toBe(true)
    expect(wrapper.text()).toContain('转换设置')
  })

  it('displays output folder', async () => {
    const mockStore = {
      outputFolder: '/test/output',
      settings: { format: 'jpeg', quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    }
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any)

    const wrapper = mount(SettingsPanel, {
      global: {
        stubs: {
          Button: {
            template: '<button><slot /></button>',
          },
          Slider: {
            template: '<div class="slider"><slot /></div>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('/test/output')
  })

  it('displays format options', async () => {
    const mockStore = {
      outputFolder: '/test/output',
      settings: { format: 'jpeg', quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    }
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any)

    const wrapper = mount(SettingsPanel, {
      global: {
        stubs: {
          Button: {
            template: '<button><slot /></button>',
          },
          Slider: {
            template: '<div class="slider"><slot /></div>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('JPEG')
    expect(wrapper.text()).toContain('PNG')
    expect(wrapper.text()).toContain('WebP')
  })

  it('displays quality slider', async () => {
    const mockStore = {
      outputFolder: '/test/output',
      settings: { format: 'jpeg', quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    }
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any)

    const wrapper = mount(SettingsPanel, {
      global: {
        stubs: {
          Button: {
            template: '<button><slot /></button>',
          },
          Slider: {
            template: '<div class="slider"><slot /></div>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('输出质量')
    expect(wrapper.text()).toContain('90')
  })
})
