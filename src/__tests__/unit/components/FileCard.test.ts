import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount } from "@vue/test-utils";
import FileCard from "@/components/FileCard.vue";
import { useConversionStore } from "@/stores/conversionStore";
import { createPinia, setActivePinia } from "pinia";
import type { FileItem } from "@/types";

vi.mock("@/stores/conversionStore", () => ({
  useConversionStore: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-log", () => ({
  debug: vi.fn(),
  info: vi.fn(),
  warn: vi.fn(),
  error: vi.fn(),
}));

const baseStubs = {
  Tooltip: true,
  TooltipContent: true,
  TooltipTrigger: true,
};

function makeStore() {
  return {
    isConverting: false,
    isStopping: false,
    removePath: vi.fn(),
    removeCompletedFile: vi.fn(),
  };
}

function mountCard(file: FileItem, flags: Record<string, boolean> = {}) {
  return mount(FileCard, {
    props: { file, ...flags },
    global: { stubs: baseStubs },
  });
}

describe("FileCard.vue - 已完成展示源文件名与生成文件名", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
    vi.mocked(useConversionStore).mockReturnValue(
      makeStore() as unknown as ReturnType<typeof useConversionStore>
    );
  });

  it("已完成项显示 源文件名 -> 生成文件名（含加序号兜底输出）", () => {
    const wrapper = mountCard(
      {
        name: "photo.heic",
        path: "/in/photo.heic",
        size: 1024,
        convertedFilePath: "/out/photo (1).jpg",
      },
      { isCompletedFile: true }
    );
    expect(wrapper.text()).toContain("photo.heic → photo (1).jpg");
  });

  it("已完成项无输出路径回退时只显示源文件名，不出现悬空箭头", () => {
    const wrapper = mountCard(
      { name: "photo.heic", path: "/in/photo.heic", size: 1024 },
      { isCompletedFile: true }
    );
    expect(wrapper.text()).toContain("photo.heic");
    expect(wrapper.text()).not.toContain("→");
  });

  it("待转换项只显示源文件名", () => {
    const wrapper = mountCard(
      { name: "photo.heic", path: "/in/photo.heic", size: 1024 },
      { isTaskFile: true }
    );
    expect(wrapper.text()).toContain("photo.heic");
    expect(wrapper.text()).not.toContain("→");
  });

  it("完整字符串同时可悬停可见（title 属性）", () => {
    const wrapper = mountCard(
      {
        name: "photo.heic",
        path: "/in/photo.heic",
        size: 1024,
        convertedFilePath: "/out/photo (1).jpg",
      },
      { isCompletedFile: true }
    );
    expect(wrapper.find("p[title]").attributes("title")).toBe("photo.heic → photo (1).jpg");
  });
});
