import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { i18n, setStoredLocale, readStoredLocale, resolveLocale } from "@/i18n";
import type { Locale } from "@/i18n/catalog";

const STORAGE_KEY = "heic-converter.locale";

const currentLocale = () => i18n.global.locale.value as string;

describe("locale 持久化与后端下发", () => {
  beforeEach(() => {
    localStorage.clear();
    vi.clearAllMocks();
    setStoredLocale("system");
    localStorage.removeItem(STORAGE_KEY);
  });

  it("未设置时默认跟随系统（system 哨兵 + 解析）", () => {
    expect(readStoredLocale()).toBe("system");
    // jsdom 的 navigator.language 为 en-US
    expect(resolveLocale("system")).toBe("en");
  });

  it("setStoredLocale 写入 localStorage、应用到 i18n 并 invoke set_locale", () => {
    setStoredLocale("zh-Hant");
    expect(localStorage.getItem(STORAGE_KEY)).toBe("zh-Hant");
    expect(currentLocale()).toBe("zh-Hant");
    expect(vi.mocked(invoke)).toHaveBeenCalledWith("set_locale", {
      locale: "zh-Hant",
    });
  });

  it("启动时从 localStorage 恢复所选语言", () => {
    localStorage.setItem(STORAGE_KEY, "zh-Hans");
    expect(resolveLocale(readStoredLocale())).toBe("zh-Hans");
    expect(readStoredLocale()).toBe("zh-Hans");
  });

  it("非法存储值回退 system", () => {
    localStorage.setItem(STORAGE_KEY, "klingon");
    expect(readStoredLocale()).toBe("system");
  });

  it("setLocale 直接切换 i18n 渲染语言（种子串变化）", () => {
    const t = i18n.global.t;
    setStoredLocale("zh-Hans");
    expect(t("settings.language")).toBe("语言");
    setStoredLocale("en");
    expect(t("settings.language")).toBe("Language");
    expect(currentLocale() as Locale).toBe("en");
  });
});
