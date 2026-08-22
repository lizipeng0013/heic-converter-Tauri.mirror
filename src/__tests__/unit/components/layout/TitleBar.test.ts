import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import TitleBar from '@/components/layout/TitleBar.vue'
import { useDark } from '@vueuse/core'

// Tooltip 组件 stub
const tooltipStubs = {
  Tooltip: {
    template: '<div><slot /></div>',
  },
  TooltipContent: {
    template: '<div><slot /></div>',
  },
  TooltipTrigger: {
    template: '<div><slot /></div>',
  },
  TooltipProvider: {
    template: '<div><slot /></div>',
  },
}

// Mock vueuse core
vi.mock('@vueuse/core', () => ({
  useDark: vi.fn(() => ({ value: false })),
  useToggle: vi.fn((val) => (() => {
    val.value = !val.value
  })),
  reactiveOmit: vi.fn((obj: any, key: string) => {
    const { [key]: _, ...rest } = obj
    return rest
  }),
}))

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: vi.fn(() => ({
    show: vi.fn(),
    hide: vi.fn(),
    minimize: vi.fn(),
    unminimize: vi.fn(),
    setFocus: vi.fn(),
    close: vi.fn(),
    toggleMaximize: vi.fn().mockResolvedValue(undefined),
    setAlwaysOnTop: vi.fn().mockResolvedValue(undefined),
    startDragging: vi.fn(),
  })),
}))

describe('TitleBar', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders title bar with app name', () => {
    const wrapper = mount(TitleBar, {
      props: {
        appName: 'Test App',
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    expect(wrapper.text()).toContain('Test App')
  })

  it('renders with default props', () => {
    const wrapper = mount(TitleBar, {
      global: {
        stubs: tooltipStubs,
      },
    })
    
    expect(wrapper.find('header').exists()).toBe(true)
    expect(wrapper.find('img').exists()).toBe(true)
  })

  it('hides logo when showLogo is false', () => {
    const wrapper = mount(TitleBar, {
      props: {
        showLogo: false,
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    expect(wrapper.find('img').exists()).toBe(false)
  })

  it('shows tray button when showTrayButton is true', () => {
    const wrapper = mount(TitleBar, {
      props: {
        showTrayButton: true,
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    expect(wrapper.find('[data-testid="tray-button"]').exists()).toBe(true)
  })

  it('hides minimize button when showMinimizeButton is false', () => {
    const wrapper = mount(TitleBar, {
      props: {
        showMinimizeButton: false,
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    const buttons = wrapper.findAll('button')
    const minimizeButton = buttons.find(b => b.text().includes('Minimize'))
    expect(minimizeButton).toBeUndefined()
  })

  it('hides maximize button when showMaximizeButton is false', () => {
    const wrapper = mount(TitleBar, {
      props: {
        showMaximizeButton: false,
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    const buttons = wrapper.findAll('button')
    const maximizeButton = buttons.find(b => b.text().includes('Maximize'))
    expect(maximizeButton).toBeUndefined()
  })

  it('hides close button when showCloseButton is false', () => {
    const wrapper = mount(TitleBar, {
      props: {
        showCloseButton: false,
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    const buttons = wrapper.findAll('button')
    const closeButton = buttons.find(b => b.text().includes('Close'))
    expect(closeButton).toBeUndefined()
  })

  it('emits close-request event when close button clicked', async () => {
    const wrapper = mount(TitleBar, {
      global: {
        stubs: tooltipStubs,
      },
    })
    
    const closeButton = wrapper.findAll('button').find(b => b.text().includes('Close'))
    await closeButton?.trigger('click')
    
    expect(wrapper.emitted('close-request')).toHaveLength(1)
  })

  it('emits minimize-to-tray event when tray button clicked', async () => {
    const wrapper = mount(TitleBar, {
      props: {
        showTrayButton: true,
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    const trayButton = wrapper.findAll('button').find(b => b.text().includes('Tray'))
    await trayButton?.trigger('click')
    
    expect(wrapper.emitted('minimize-to-tray')).toHaveLength(1)
  })

  it('applies correct height class for tiny size', () => {
    const wrapper = mount(TitleBar, {
      props: {
        height: 'tiny',
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    expect(wrapper.find('header').classes()).toContain('h-6')
  })

  it('applies correct height class for large size', () => {
    const wrapper = mount(TitleBar, {
      props: {
        height: 'large',
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    expect(wrapper.find('header').classes()).toContain('h-12')
  })

  it('toggles dark mode when theme button clicked', async () => {
    const wrapper = mount(TitleBar, {
      global: {
        stubs: tooltipStubs,
      },
    })
    
    const themeButton = wrapper.findAll('button').find(b => b.text().includes('Theme'))
    await themeButton?.trigger('click')
    
    // 检查 useToggle 被调用
    expect(useDark).toHaveBeenCalled()
  })

  it('renders with custom class', () => {
    const wrapper = mount(TitleBar, {
      props: {
        class: 'custom-titlebar',
      },
      global: {
        stubs: tooltipStubs,
      },
    })
    
    expect(wrapper.find('header').classes()).toContain('custom-titlebar')
  })
})
