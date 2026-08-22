import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import FileListArea from '@/views/file/FileListArea.vue'
import { useConversionStore } from '@/stores/conversionStore'
import { createPinia, setActivePinia } from 'pinia'

vi.mock('@/stores/conversionStore', () => ({
  useConversionStore: vi.fn(),
}))

vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}))

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
  TauriEvent: {},
  UnlistenFn: vi.fn(),
}))

vi.mock('@tanstack/vue-virtual', () => ({
  useVirtualizer: vi.fn(() => ({
    value: {
      getVirtualItems: () => [],
      getTotalSize: () => 0,
    },
  })),
}))

describe('FileListArea.vue', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('renders file list area', () => {
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 0, waiting: 0, processing: 0, completed: 0, failed: 0 },
      isConverting: false,
      isReadyForConversion: false,
      activeTab: 'pending',
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    }
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any)

    const wrapper = mount(FileListArea, {
      global: {
        stubs: {
          Upload: true,
          CheckCircle2: true,
          Button: {
            template: '<button><slot /></button>',
          },
          Tabs: {
            template: '<div><slot /></div>',
          },
          TabsList: {
            template: '<div><slot /></div>',
          },
          TabsTrigger: {
            template: '<button><slot /></button>',
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    })

    expect(wrapper.find('.bg-card').exists()).toBe(true)
    expect(wrapper.text()).toContain('转换文件')
  })

  it('displays empty state when no files', () => {
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 0, waiting: 0, processing: 0, completed: 0, failed: 0 },
      isConverting: false,
      isReadyForConversion: false,
      activeTab: 'pending',
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    }
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any)

    const wrapper = mount(FileListArea, {
      global: {
        stubs: {
          Upload: true,
          Button: {
            template: '<button><slot /></button>',
          },
          Tabs: {
            template: '<div><slot /></div>',
          },
          TabsList: {
            template: '<div><slot /></div>',
          },
          TabsTrigger: {
            template: '<button><slot /></button>',
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('拖拽文件到此处或')
    expect(wrapper.text()).toContain('添加文件')
  })

  it('displays add files button', () => {
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 0, waiting: 0, processing: 0, completed: 0, failed: 0 },
      isConverting: false,
      isReadyForConversion: false,
      activeTab: 'pending',
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    }
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any)

    const wrapper = mount(FileListArea, {
      global: {
        stubs: {
          Upload: true,
          Button: {
            template: '<button><slot /></button>',
          },
          Tabs: {
            template: '<div><slot /></div>',
          },
          TabsList: {
            template: '<div><slot /></div>',
          },
          TabsTrigger: {
            template: '<button><slot /></button>',
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('添加文件')
  })

  it('displays conversion stats', () => {
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 5, waiting: 3, processing: 2, completed: 0, failed: 0 },
      isConverting: true,
      isReadyForConversion: true,
      activeTab: 'pending',
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    }
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any)

    const wrapper = mount(FileListArea, {
      global: {
        stubs: {
          Upload: true,
          Button: {
            template: '<button><slot /></button>',
          },
          Tabs: {
            template: '<div><slot /></div>',
          },
          TabsList: {
            template: '<div><slot /></div>',
          },
          TabsTrigger: {
            template: '<button><slot /></button>',
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    })

    expect(wrapper.text()).toContain('总计')
    expect(wrapper.text()).toContain('5')
  })
})
