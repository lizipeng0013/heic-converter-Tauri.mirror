import { describe, it, expect, beforeEach, vi } from "vitest";
import { useConversionStore } from "@/stores/conversionStore";
import { createPinia, setActivePinia } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { setTestLocale } from "@/i18n";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  ask: vi.fn(),
  message: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-fs", () => ({
  stat: vi.fn().mockResolvedValue({ size: 1024 }),
}));

vi.mock("@tauri-apps/plugin-log", () => ({
  debug: vi.fn(),
  warn: vi.fn(),
  error: vi.fn(),
}));

describe("conversionStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  describe("addPaths", () => {
    it("should add HEIC files correctly", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test.heic"]);

      expect(store.files).toHaveLength(1);
      expect(store.files[0].path).toBe("/path/to/test.heic");
      expect(store.files[0].name).toBe("test.heic");
    });

    it("should filter non-HEIC files", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test.jpg"]);

      expect(store.files).toHaveLength(0);
    });

    it("should filter HEIF files", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test.heif"]);

      expect(store.files).toHaveLength(1);
      expect(store.files[0].name).toBe("test.heif");
    });

    it("should handle duplicate files", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test.heic"]);
      await store.addPaths(["/path/to/test.heic"]);

      expect(store.files).toHaveLength(1);
    });

    it("should handle empty paths array", async () => {
      const store = useConversionStore();

      await expect(store.addPaths([])).resolves.not.toThrow();
      expect(store.files).toHaveLength(0);
    });

    it("should handle mixed valid and invalid files", async () => {
      const store = useConversionStore();

      await store.addPaths([
        "/path/to/valid1.heic",
        "/path/to/invalid.jpg",
        "/path/to/valid2.heic",
      ]);

      expect(store.files).toHaveLength(2);
      expect(store.files[0].name).toBe("valid1.heic");
      expect(store.files[1].name).toBe("valid2.heic");
    });
  });

  describe("removePath", () => {
    it("should remove file from files list", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test.heic"]);
      expect(store.files).toHaveLength(1);

      store.removePath("/path/to/test.heic");
      expect(store.files).toHaveLength(0);
    });

    it("should remove file from errorFiles list", async () => {
      const store = useConversionStore();

      // 先添加文件
      await store.addPaths(["/path/to/test.heic"]);
      expect(store.files).toHaveLength(1);

      // 然后标记为错误
      store.updateFileError("/path/to/test.heic", "Test error");

      expect(store.errorFiles).toHaveLength(1);

      store.removePath("/path/to/test.heic");
      expect(store.errorFiles).toHaveLength(0);
    });
  });

  describe("clearPaths", () => {
    it("should clear all files", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test1.heic", "/path/to/test2.heic"]);
      expect(store.files).toHaveLength(2);

      store.clearPaths();
      expect(store.files).toHaveLength(0);
      expect(store.isReadyForConversion).toBe(false);
    });
  });

  describe("clearErrors", () => {
    it("should clear only errorFiles, keeping pending queue and conversion state", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/ok.heic", "/path/to/bad.heic"]);
      store.updateFileError("/path/to/bad.heic", "转换失败");
      expect(store.errorFiles).toHaveLength(1);
      expect(store.files).toHaveLength(1);

      store.isConverting = true;
      store.clearErrors();

      expect(store.errorFiles).toHaveLength(0);
      // 待转换队列与转换状态不受影响
      expect(store.files).toHaveLength(1);
      expect(store.isConverting).toBe(true);
    });
  });

  describe("updateFileSuccess", () => {
    it("should move file to completed list", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test.heic"]);
      expect(store.files).toHaveLength(1);

      store.updateFileSuccess("/path/to/test.heic", "/output/test.jpg");

      expect(store.files).toHaveLength(0);
      expect(store.completedFiles).toHaveLength(1);
      expect(store.completedFiles[0].convertedFilePath).toBe("/output/test.jpg");
    });
  });

  describe("updateFileError", () => {
    it("should move file to error list", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test.heic"]);
      expect(store.files).toHaveLength(1);

      store.updateFileError("/path/to/test.heic", "Conversion failed");

      expect(store.files).toHaveLength(0);
      expect(store.errorFiles).toHaveLength(1);
      expect(store.errorFiles[0].errorMessage).toBe("Conversion failed");
    });
  });

  describe("settings", () => {
    it("should update settings", () => {
      const store = useConversionStore();

      store.updateSettings({ format: "png" });

      expect(store.settings.format).toBe("png");
    });

    it("should update quality", () => {
      const store = useConversionStore();

      store.updateSettings({ quality: [80] });

      expect(store.settings.quality[0]).toBe(80);
    });
  });

  describe("outputFolder", () => {
    it("should set output folder", () => {
      const store = useConversionStore();

      store.setOutputFolder("/custom/output");

      expect(store.outputFolder).toBe("/custom/output");
    });

    it("should clear output folder when set to null", () => {
      const store = useConversionStore();

      store.setOutputFolder("/custom/output");
      store.setOutputFolder(null);

      expect(store.outputFolder).toBeNull();
    });

    it("should confirm output folder with the backend", async () => {
      const store = useConversionStore();
      vi.mocked(invoke).mockClear();

      store.setOutputFolder("/custom/output");

      await vi.waitFor(() => {
        expect(invoke).toHaveBeenCalledWith("confirm_output_folder", {
          path: "/custom/output",
        });
      });
    });
  });

  describe("stats", () => {
    it("should calculate stats correctly", async () => {
      const store = useConversionStore();

      await store.addPaths(["/path/to/test1.heic", "/path/to/test2.heic"]);

      store.updateFileSuccess("/path/to/test1.heic", "/output/test1.jpg");
      store.updateFileError("/path/to/test2.heic", "Failed");

      expect(store.stats.total).toBe(2);
      expect(store.stats.completed).toBe(1);
      expect(store.stats.failed).toBe(1);
    });
  });

  describe("tabs", () => {
    it("should set active tab", () => {
      const store = useConversionStore();

      store.setActiveTab("completed");

      expect(store.activeTab).toBe("completed");
    });
  });

  describe("groupExpansion", () => {
    it("should toggle group expansion", () => {
      const store = useConversionStore();

      expect(store.taskExpanded).toBe(true);

      store.toggleGroupExpansion("task");
      expect(store.taskExpanded).toBe(false);

      store.toggleGroupExpansion("error");
      expect(store.errorExpanded).toBe(true);
      expect(store.taskExpanded).toBe(false);
    });
  });

  it("触发错误弹窗显示当前语言文案（钉 zh-Hans / en）", async () => {
    const store = useConversionStore();
    await store.addPaths(["/path/to/test.heic"]);

    setTestLocale("zh-Hans");
    vi.mocked(invoke).mockRejectedValueOnce(new Error("boom"));
    await store.startConversion();
    expect(vi.mocked(message)).toHaveBeenLastCalledWith(
      expect.stringContaining("转换任务执行失败"),
      expect.objectContaining({
        title: expect.stringContaining("发生错误"),
        kind: "error",
      })
    );

    setTestLocale("en");
    vi.mocked(invoke).mockRejectedValueOnce(new Error("boom"));
    await store.startConversion();
    expect(vi.mocked(message)).toHaveBeenLastCalledWith(
      expect.stringContaining("Failed to start the conversion"),
      expect.objectContaining({ title: "Error", kind: "error" })
    );
  });
});
