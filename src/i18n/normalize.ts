import type { Locale } from "./catalog";

/**
 * 把裸系统语言标签归一化为内部目录键。
 * navigator.language 因 OS/浏览器而异（zh-CN、zh-TW、zh-Hans-CN、zh_TW…），
 * 永不直接等于目录键，必须手动映射：
 * 含 Hant 或地区 TW/HK/MO → zh-Hant；其余 zh* → zh-Hans；其他 → en。
 */
export function normalizeLocale(raw: string | null | undefined): Locale {
  const tag = (raw ?? "").replace(/_/g, "-").toLowerCase();
  if (!tag.startsWith("zh")) return "en";
  if (tag.includes("hant") || /^zh-(tw|hk|mo)(-|$)/.test(tag)) return "zh-Hant";
  return "zh-Hans";
}
