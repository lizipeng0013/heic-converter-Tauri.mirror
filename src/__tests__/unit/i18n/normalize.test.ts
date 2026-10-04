import { describe, it, expect } from "vitest";
import { normalizeLocale } from "@/i18n/normalize";

describe("normalizeLocale — 系统标签归一化", () => {
  it.each([
    ["zh-CN", "zh-Hans"],
    ["zh-SG", "zh-Hans"],
    ["zh", "zh-Hans"],
    ["zh-Hans", "zh-Hans"],
    ["zh-Hans-CN", "zh-Hans"],
    ["zh_CN", "zh-Hans"],
    ["zh-TW", "zh-Hant"],
    ["zh-HK", "zh-Hant"],
    ["zh-MO", "zh-Hant"],
    ["zh-Hant", "zh-Hant"],
    ["zh-Hant-TW", "zh-Hant"],
    ["zh_TW", "zh-Hant"],
    ["en-US", "en"],
    ["en", "en"],
    ["fr-FR", "en"],
    ["ja-JP", "en"],
    ["", "en"],
    [undefined, "en"],
    [null, "en"],
  ])("%s -> %s", (input, expected) => {
    expect(normalizeLocale(input)).toBe(expected);
  });
});
