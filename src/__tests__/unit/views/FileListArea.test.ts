import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount } from "@vue/test-utils";
import FileListArea from "@/views/file/FileListArea.vue";
import { useConversionStore } from "@/stores/conversionStore";
import { createPinia, setActivePinia } from "pinia";
import { setTestLocale, i18n } from "@/i18n";

vi.mock("@/stores/conversionStore", () => ({
  useConversionStore: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(),
  TauriEvent: {},
  UnlistenFn: vi.fn(),
}));

vi.mock("@tanstack/vue-virtual", () => ({
  useVirtualizer: vi.fn(() => ({
    value: {
      getVirtualItems: () => [],
      getTotalSize: () => 0,
    },
  })),
}));

describe("FileListArea.vue", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
    setTestLocale("zh-Hans");
  });

  it("renders file list area", () => {
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 0, waiting: 0, processing: 0, completed: 0, failed: 0 },
      isConverting: false,
      isReadyForConversion: false,
      activeTab: "pending",
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(FileListArea, {
      global: {
        plugins: [i18n],
        stubs: {
          Upload: true,
          CheckCircle2: true,
          Button: {
            template: "<button><slot /></button>",
          },
          Tabs: {
            template: "<div><slot /></div>",
          },
          TabsList: {
            template: "<div><slot /></div>",
          },
          TabsTrigger: {
            template: "<button><slot /></button>",
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    });

    expect(wrapper.find(".bg-card").exists()).toBe(true);
    // 检查实际渲染的文本内容
    expect(wrapper.text()).toContain("任务列表");
    expect(wrapper.text()).toContain("已完成");
  });

  it("displays empty state when no files", () => {
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 0, waiting: 0, processing: 0, completed: 0, failed: 0 },
      isConverting: false,
      isReadyForConversion: false,
      activeTab: "pending",
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(FileListArea, {
      global: {
        plugins: [i18n],
        stubs: {
          Upload: true,
          Button: {
            template: "<button><slot /></button>",
          },
          Tabs: {
            template: "<div><slot /></div>",
          },
          TabsList: {
            template: "<div><slot /></div>",
          },
          TabsTrigger: {
            template: "<button><slot /></button>",
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    });

    // 检查实际渲染的空状态文本
    expect(wrapper.text()).toContain("拖拽 HEIC 文件到此处");
  });

  it("displays add files button", () => {
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 0, waiting: 0, processing: 0, completed: 0, failed: 0 },
      isConverting: false,
      isReadyForConversion: false,
      activeTab: "pending",
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(FileListArea, {
      global: {
        plugins: [i18n],
        stubs: {
          Upload: true,
          Button: {
            template: "<button><slot /></button>",
          },
          Tabs: {
            template: "<div><slot /></div>",
          },
          TabsList: {
            template: "<div><slot /></div>",
          },
          TabsTrigger: {
            template: "<button><slot /></button>",
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    });

    // 检查实际渲染的空状态文本
    const text = wrapper.text();
    expect(text).toContain("拖拽 HEIC 文件到此处");
    expect(text).toContain("支持 .heic, .heif 格式");
  });

  it("displays conversion stats", () => {
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 5, waiting: 3, processing: 2, completed: 0, failed: 0 },
      isConverting: true,
      isReadyForConversion: true,
      activeTab: "pending",
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(FileListArea, {
      global: {
        plugins: [i18n],
        stubs: {
          Upload: true,
          Button: {
            template: "<button><slot /></button>",
          },
          Tabs: {
            template: "<div><slot /></div>",
          },
          TabsList: {
            template: "<div><slot /></div>",
          },
          TabsTrigger: {
            template: "<button><slot /></button>",
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    });

    // 检查实际渲染的统计信息
    expect(wrapper.text()).toContain("任务列表");
    expect(wrapper.text()).toContain("0");
  });

  // --- 清空按钮的转换中禁用与"清空失败"行为 ---

  const makeStore = (overrides: Record<string, unknown> = {}) => ({
    files: [],
    errorFiles: [],
    completedFiles: [],
    taskFiles: [],
    errorFilesList: [],
    stats: { total: 0, waiting: 0, processing: 0, completed: 0, failed: 0 },
    isConverting: false,
    isPreparing: false,
    isStopping: false,
    isReadyForConversion: false,
    activeTab: "pending",
    addPaths: vi.fn(),
    removePath: vi.fn(),
    clearPaths: vi.fn(),
    clearErrors: vi.fn(),
    clearCompletedFiles: vi.fn(),
    setActiveTab: vi.fn(),
    toggleGroupExpansion: vi.fn(),
    taskExpanded: true,
    errorExpanded: false,
    ...overrides,
  });

  const mountWithStore = (store: Record<string, unknown>) => {
    vi.mocked(useConversionStore).mockReturnValue(
      store as unknown as ReturnType<typeof useConversionStore>
    );
    return mount(FileListArea, {
      global: {
        plugins: [i18n],
        stubs: {
          Upload: true,
          CheckCircle2: true,
          Button: {
            template: "<button><slot /></button>",
          },
          Tabs: {
            template: "<div><slot /></div>",
          },
          TabsList: {
            template: "<div><slot /></div>",
          },
          TabsTrigger: {
            template: "<button><slot /></button>",
          },
          FileCard: {
            template: '<div class="file-card"><slot /></div>',
          },
        },
      },
    });
  };

  const findButton = (wrapper: ReturnType<typeof mountWithStore>, text: string) =>
    wrapper.findAll("button").find((b) => b.text().includes(text));

  it("disables clear-list button while converting", () => {
    const store = makeStore({
      files: [{ path: "/a.heic", name: "a.heic", size: 1 }],
      taskFiles: [{ path: "/a.heic", name: "a.heic", size: 1 }],
      activeTab: "pending",
      isConverting: true,
    });
    const wrapper = mountWithStore(store);

    const btn = findButton(wrapper, "清空列表");
    expect(btn).toBeDefined();
    expect(btn!.attributes("disabled")).toBeDefined();
  });

  it("enables clear-list button when not converting", () => {
    const store = makeStore({
      files: [{ path: "/a.heic", name: "a.heic", size: 1 }],
      taskFiles: [{ path: "/a.heic", name: "a.heic", size: 1 }],
      activeTab: "pending",
      isConverting: false,
    });
    const wrapper = mountWithStore(store);

    const btn = findButton(wrapper, "清空列表");
    expect(btn).toBeDefined();
    expect(btn!.attributes("disabled")).toBeUndefined();
  });

  it("clear-errors button clears only the error list", async () => {
    const store = makeStore({
      errorFiles: [{ path: "/b.heic", name: "b.heic", size: 1 }],
      errorFilesList: [{ path: "/b.heic", name: "b.heic", size: 1 }],
      activeTab: "error",
      isConverting: false,
    });
    const wrapper = mountWithStore(store);

    const btn = findButton(wrapper, "清空失败");
    expect(btn).toBeDefined();
    await btn!.trigger("click");

    // 只调用 clearErrors，绝不能调用会连带清空待转换队列的 clearPaths
    expect(store.clearErrors).toHaveBeenCalledTimes(1);
    expect(store.clearPaths).not.toHaveBeenCalled();
  });

  it("disables clear-errors button while converting", () => {
    const store = makeStore({
      errorFiles: [{ path: "/b.heic", name: "b.heic", size: 1 }],
      errorFilesList: [{ path: "/b.heic", name: "b.heic", size: 1 }],
      activeTab: "error",
      isConverting: true,
    });
    const wrapper = mountWithStore(store);

    const btn = findButton(wrapper, "清空失败");
    expect(btn).toBeDefined();
    expect(btn!.attributes("disabled")).toBeDefined();
  });

  it("renders English copy when locale is pinned to en", () => {
    setTestLocale("en");
    const mockStore = {
      files: [],
      errorFiles: [],
      completedFiles: [],
      taskFiles: [],
      errorFilesList: [],
      stats: { total: 0, waiting: 0, processing: 0, completed: 0, failed: 0 },
      isConverting: false,
      isReadyForConversion: false,
      activeTab: "pending",
      addPaths: vi.fn(),
      removePath: vi.fn(),
      clearPaths: vi.fn(),
      setActiveTab: vi.fn(),
      toggleGroupExpansion: vi.fn(),
      taskExpanded: true,
      errorExpanded: false,
    };
    vi.mocked(useConversionStore).mockReturnValue(
      mockStore as unknown as ReturnType<typeof useConversionStore>
    );
    const wrapper = mount(FileListArea, {
      global: {
        plugins: [i18n],
        stubs: {
          Upload: true,
          CheckCircle2: true,
          Button: { template: "<button><slot /></button>" },
          Tabs: { template: "<div><slot /></div>" },
          TabsList: { template: "<div><slot /></div>" },
          TabsTrigger: { template: "<div><slot /></div>" },
          FileCard: true,
        },
      },
    });
    expect(wrapper.text()).toContain("Tasks");
    expect(wrapper.text()).toContain("Drop HEIC files here");
  });
});
