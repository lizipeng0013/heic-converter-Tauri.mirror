import { createI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";
import { buildMessages, LOCALES, type Locale } from "./catalog";
import { normalizeLocale } from "./normalize";

export type StoredLocale = "system" | Locale;

const STORAGE_KEY = "heic-converter.locale";
const VALID: string[] = [...LOCALES, "system"];

export function readStoredLocale(): StoredLocale {
  try {
    const value = localStorage.getItem(STORAGE_KEY);
    if (value && VALID.includes(value)) return value as StoredLocale;
  } catch {
    // localStorage 不可用时回退系统跟随
  }
  return "system";
}

export function resolveLocale(stored: StoredLocale): Locale {
  if (stored !== "system") return stored;
  return normalizeLocale(typeof navigator !== "undefined" ? navigator.language : undefined);
}

export const i18n = createI18n({
  legacy: false,
  globalInjection: true,
  locale: resolveLocale(readStoredLocale()),
  fallbackLocale: "en",
  messages: buildMessages(),
});

function applyLocale(locale: Locale): void {
  i18n.global.locale.value = locale;
  pushLocaleToBackend(locale);
}

// 下发失败自动重试一次；仍失败则记录（前端 locale 已生效，后端在下次切换时会再次对齐）
function pushLocaleToBackend(locale: Locale): void {
  void Promise.resolve(invoke("set_locale", { locale })).catch(() =>
    Promise.resolve(invoke("set_locale", { locale })).catch(
      (e) => void error(`下发语言设置到后端失败: ${e}`)
    )
  );
}

/** 持久化语言选择（含 system 哨兵）并立即应用 + 下发 Rust */
export function setStoredLocale(stored: StoredLocale): void {
  try {
    localStorage.setItem(STORAGE_KEY, stored);
  } catch {
    // 持久化失败不影响本次会话
  }
  applyLocale(resolveLocale(stored));
}

/** 测试专用：钉住渲染语言，不写存储、不 invoke */
export function setTestLocale(locale: Locale): void {
  i18n.global.locale.value = locale;
}

/** 应用启动时调用：应用初始语言并监听系统语言变化（仅哨兵模式下生效） */
export function initLocale(): void {
  applyLocale(resolveLocale(readStoredLocale()));
  window.addEventListener("languagechange", () => {
    if (readStoredLocale() === "system") applyLocale(resolveLocale("system"));
  });
}
