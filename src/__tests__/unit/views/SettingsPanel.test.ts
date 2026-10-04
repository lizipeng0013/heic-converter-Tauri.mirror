import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount } from "@vue/test-utils";
import SettingsPanel from "@/views/settings/SettingsPanel.vue";
import { useConversionStore } from "@/stores/conversionStore";
import { createPinia, setActivePinia } from "pinia";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { setTestLocale, i18n } from "@/i18n";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@/stores/conversionStore", () => ({
  useConversionStore: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

vi.mock("@tauri-apps/api/path", () => ({
  downloadDir: vi.fn(),
}));

describe("SettingsPanel.vue", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
    localStorage.clear();
    setTestLocale("zh-Hans");
  });

  it("renders settings panel", async () => {
    const mockStore = {
      outputFolder: "/test/output",
      settings: { format: "jpeg", quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);
    vi.mocked(openDialog).mockResolvedValue("/new/output");

    const wrapper = mount(SettingsPanel, {
      global: {
        plugins: [i18n],
        stubs: {
          Settings2: true,
          FolderOpen: true,
          ChevronDown: true,
          Square: true,
          Button: {
            template: "<button><slot /></button>",
          },
          Slider: {
            template: '<div class="slider"><slot /></div>',
          },
          Tooltip: {
            template: "<div><slot /></div>",
          },
          TooltipContent: {
            template: "<div><slot /></div>",
          },
          TooltipTrigger: {
            template: "<div><slot /></div>",
          },
          TooltipProvider: {
            template: "<div><slot /></div>",
          },
        },
      },
    });

    expect(wrapper.find(".bg-card").exists()).toBe(true);
    expect(wrapper.text()).toContain("转换设置");
  });

  it("切换语言后标题立即变化、写入存储并下发 set_locale", async () => {
    const mockStore = {
      outputFolder: "/test/output",
      settings: { format: "jpeg", quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(SettingsPanel, {
      global: {
        plugins: [i18n],
        stubs: {
          Settings2: true,
          FolderOpen: true,
          ChevronDown: true,
          Square: true,
          Button: { template: "<button><slot /></button>" },
          Slider: { template: '<div class="slider"><slot /></div>' },
          Tooltip: { template: "<div><slot /></div>" },
          TooltipContent: { template: "<div><slot /></div>" },
          TooltipTrigger: { template: "<div><slot /></div>" },
          TooltipProvider: { template: "<div><slot /></div>" },
        },
      },
    });

    expect(wrapper.text()).toContain("转换设置");
    await wrapper.find("#language-select").trigger("click");
    const enOption = wrapper.findAll("button").find((b) => b.text().trim() === "English");
    expect(enOption).toBeTruthy();
    await enOption!.trigger("click");
    expect(wrapper.text()).toContain("Conversion Settings");
    expect(localStorage.getItem("heic-converter.locale")).toBe("en");
    expect(vi.mocked(invoke)).toHaveBeenCalledWith("set_locale", { locale: "en" });
  });

  it("displays output folder", async () => {
    const mockStore = {
      outputFolder: "/test/output",
      settings: { format: "jpeg", quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(SettingsPanel, {
      global: {
        plugins: [i18n],
        stubs: {
          Button: {
            template: "<button><slot /></button>",
          },
          Slider: {
            template: '<div class="slider"><slot /></div>',
          },
          Tooltip: {
            template: "<div><slot /></div>",
          },
          TooltipContent: {
            template: "<div><slot /></div>",
          },
          TooltipTrigger: {
            template: "<div><slot /></div>",
          },
          TooltipProvider: {
            template: "<div><slot /></div>",
          },
        },
      },
    });

    expect(wrapper.text()).toContain("/test/output");
  });

  it("displays format options", async () => {
    const mockStore = {
      outputFolder: "/test/output",
      settings: { format: "jpeg", quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(SettingsPanel, {
      global: {
        plugins: [i18n],
        stubs: {
          Button: {
            template: "<button><slot /></button>",
          },
          Slider: {
            template: '<div class="slider"><slot /></div>',
          },
          Tooltip: {
            template: "<div><slot /></div>",
          },
          TooltipContent: {
            template: "<div><slot /></div>",
          },
          TooltipTrigger: {
            template: "<div><slot /></div>",
          },
          TooltipProvider: {
            template: "<div><slot /></div>",
          },
        },
      },
    });

    expect(wrapper.text()).toContain("JPEG");
    expect(wrapper.text()).toContain("PNG");
    expect(wrapper.text()).toContain("WebP");
  });

  it("打开更多格式下拉并选择非常用格式（DropdownMenu 交互）", async () => {
    const mockStore = {
      outputFolder: "/test/output",
      settings: { format: "jpeg", quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    };
    vi.mocked(useConversionStore).mockReturnValue(
      mockStore as unknown as ReturnType<typeof useConversionStore>
    );

    // attachTo document.body：外部点击需冒泡到 document（默认挂载在游离节点，事件到不了）
    const wrapper = mount(SettingsPanel, {
      attachTo: document.body,
      global: {
        plugins: [i18n],
        stubs: {
          Button: { template: "<button><slot /></button>" },
          Slider: { template: '<div class="slider"><slot /></div>' },
          Tooltip: { template: "<div><slot /></div>" },
          TooltipContent: { template: "<div><slot /></div>" },
          TooltipTrigger: { template: "<div><slot /></div>" },
          TooltipProvider: { template: "<div><slot /></div>" },
        },
      },
    });

    // 常用格式选中时，触发按钮显示“更多”（文案随当前语言）
    const more = i18n.global.t("settings.more");
    const trigger = wrapper.findAll("button").find((b) => b.text().trim() === more);
    expect(trigger).toBeTruthy();
    await trigger!.trigger("click");

    // 菜单渲染非常用格式项，选择 BMP 写入 store
    const bmp = wrapper.findAll("button").find((b) => b.text().trim() === "BMP");
    expect(bmp).toBeTruthy();
    await bmp!.trigger("click");
    expect(mockStore.updateSettings).toHaveBeenCalledWith({ format: "bmp" });
    // 选择后菜单关闭
    expect(wrapper.findAll("button").find((b) => b.text().trim() === "BMP")).toBeUndefined();

    // 重新打开后，点击组件外部（标题）应关闭菜单
    await trigger!.trigger("click");
    expect(wrapper.findAll("button").find((b) => b.text().trim() === "BMP")).toBeTruthy();
    await wrapper.find("h2").trigger("click");
    await wrapper.vm.$nextTick();
    expect(wrapper.findAll("button").find((b) => b.text().trim() === "BMP")).toBeUndefined();
    wrapper.unmount();
  });

  it("displays quality slider", async () => {
    const mockStore = {
      outputFolder: "/test/output",
      settings: { format: "jpeg", quality: [90] },
      stats: { total: 0, completed: 0, failed: 0 },
      updateSettings: vi.fn(),
      setOutputFolder: vi.fn(),
    };
    vi.mocked(useConversionStore).mockReturnValue(
      mockStore as unknown as ReturnType<typeof useConversionStore>
    );

    const wrapper = mount(SettingsPanel, {
      global: {
        plugins: [i18n],
        stubs: {
          Button: {
            template: "<button><slot /></button>",
          },
          Slider: {
            template: '<div class="slider"><slot /></div>',
          },
          Tooltip: {
            template: "<div><slot /></div>",
          },
          TooltipContent: {
            template: "<div><slot /></div>",
          },
          TooltipTrigger: {
            template: "<div><slot /></div>",
          },
          TooltipProvider: {
            template: "<div><slot /></div>",
          },
        },
      },
    });

    // 检查实际渲染的质量文本（可能是 "图片质量" 或 "输出质量"）
    const text = wrapper.text();
    expect(text).toContain("90");
    expect(text).toContain("质量");
  });
});
