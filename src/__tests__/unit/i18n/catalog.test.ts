import { describe, it, expect } from "vitest";
import { catalog, buildMessages, LOCALES } from "@/i18n/catalog";

describe("i18n 消息目录", () => {
  it("每条目三种语言齐全且非空", () => {
    const entries = Object.entries(catalog);
    expect(entries.length).toBeGreaterThan(0);
    for (const [key, entry] of entries) {
      for (const locale of LOCALES) {
        expect(entry[locale], `${key} 缺 ${locale} 翻译`).toBeTruthy();
      }
    }
  });

  it("buildMessages 产出三套按 locale 分组的消息且键集一致", () => {
    const messages = buildMessages();
    const keySets = LOCALES.map((l) => Object.keys(messages[l]).sort());
    expect(keySets[0].length).toBeGreaterThan(0);
    expect(keySets[1]).toEqual(keySets[0]);
    expect(keySets[2]).toEqual(keySets[0]);
  });
});
