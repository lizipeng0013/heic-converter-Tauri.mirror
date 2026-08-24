import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount } from "@vue/test-utils";
import StatusBar from "@/views/status/StatusBar.vue";
import { useConversionStore } from "@/stores/conversionStore";
import { createPinia, setActivePinia } from "pinia";

vi.mock("@/stores/conversionStore", () => ({
  useConversionStore: vi.fn(),
}));

describe("StatusBar.vue", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("renders status bar", () => {
    const mockStore = {
      spendTime: null,
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(StatusBar, {
      global: {
        stubs: {
          Clock: true,
        },
      },
    });

    expect(wrapper.find(".h-8").exists()).toBe(true);
    expect(wrapper.text()).toContain("总耗时");
  });

  it("displays spend time when available", () => {
    const mockStore = {
      spendTime: 123.4,
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(StatusBar, {
      global: {
        stubs: {
          Clock: true,
        },
      },
    });

    expect(wrapper.text()).toContain("123.4s");
  });

  it("displays placeholder when spend time is null", () => {
    const mockStore = {
      spendTime: null,
    };
    vi.mocked(useConversionStore).mockReturnValue(mockStore as any);

    const wrapper = mount(StatusBar, {
      global: {
        stubs: {
          Clock: true,
        },
      },
    });

    expect(wrapper.text()).toContain("...");
  });
});
