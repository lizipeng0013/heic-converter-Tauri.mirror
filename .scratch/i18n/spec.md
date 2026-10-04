# 多语言支持（i18n）——英文 / 简体中文 / 繁体中文（港澳台）

**Status:** ready-for-agent

## Problem Statement

应用前端与 Rust 侧所有面向用户的文案（按钮、标签、弹窗、错误提示、系统通知）目前全部硬编码为简体中文，只有中文用户能正常使用。英文用户看不懂界面，港澳台繁中用户看到的也是简体。切换语言的能力完全不存在：没有 i18n 库、没有语言设置项、没有语言持久化。

## Solution

应用提供三种语言：英文（作为源语言，源串待补写）、简体中文、繁体中文（港澳台用词）。界面文案、失败列表的错误详情、命令层错误弹窗、系统通知全部按所选语言显示。默认跟随系统语言，用户可在设置面板手动切换，选择在重启后仍然生效；切换后界面立即变化，无需重启应用。

## User Stories

1. As an English-speaking user, I want the entire UI (buttons, tabs, badges, tooltips, dialogs) in English, so that I can use the app without knowing Chinese.
2. As a Simplified Chinese user, I want the UI in Simplified Chinese, so that the app reads naturally for me.
3. As a Hong Kong / Taiwan / Macau user, I want Traditional Chinese with regional (港澳台) terminology, so that the wording matches what I use daily (e.g. 檔案/資料夾 rather than simplified calques).
4. As a first-launch user, I want the app to follow my operating system language automatically, so that I don't have to configure anything before use.
5. As a user, I want to switch language in the settings panel, so that I can override the system default at any time.
6. As a user, I want the language options displayed in their own language (English / 简体中文 / 繁體中文), so that I can find my language even when the UI is in an unfamiliar one.
7. As a user, I want my language choice persisted across app restarts, so that I don't have to re-select it every time.
8. As a user, I want the language change applied immediately after selection (labels, tooltips, pending texts), so that I don't need to restart the app.
9. As a user who never manually selected a language, I want the app to keep following system language changes on next launch, so that the app stays consistent with the OS.
10. As a user, I want conversion failures surfaced as dialogs and failed-list tooltips in my language, so that I understand what went wrong without guessing.
11. As a user, I want errors raised by the backend command layer (no files selected, invalid input, output not confirmed, batch already running, size/dimension limits, output conflicts) rendered in my language, so that Rust-side failures are as readable as UI strings.
12. As a user, I want system notifications ("转换完成" etc.) in my language, so that a notification is meaningful even when the window is minimized.
13. As a user, I want notification language to match my app language choice regardless of OS language, so that an explicit override is respected everywhere.
14. As a user, I want filenames, paths, numbers and file sizes inside messages left untouched by translation, so that dynamic content stays accurate.
15. As a user, I want file dialogs and OS-level surfaces to remain native, so that the system's own localization keeps working.
16. As an English user, I want English source strings written as the authoritative text (not machine-translated from Chinese), so that the primary wording is idiomatic.
17. As a translator, I want a single catalog per side (frontend and Rust) with one key per message, so that adding or correcting a translation touches exactly one place.
18. As a developer, I want all frontend UI strings accessed through the i18n layer rather than inline literals, so that a grep for hard-coded UI text finds nothing.
19. As a developer, I want backend user-facing strings (error variants, command errors, notifications) generated from one Rust message catalog keyed by locale, so that Rust never hard-codes a language.
20. As a developer, I want the Rust-side locale delivered from the frontend (the single source of the user's choice), so that frontend and backend never disagree about the current language.
21. As a developer, I want a parity test asserting all three locales expose exactly the same key set with no empty values (both frontend and Rust), so that a missing translation fails CI instead of leaking raw keys to users.
22. As a developer, I want existing tests that assert Chinese text pinned to an explicit locale, so that test results don't depend on the host machine's language.
23. As a developer, I want the initial Rust locale to be a safe default until the frontend sets it, so that no command or notification can observe an unset locale.
24. As a maintainer, I want adding a fourth locale later to be catalog-only work (no code changes), so that the architecture doesn't need revisiting.
25. As a maintainer, I want logs (Rust log records and frontend plugin-log calls) left as-is, so that troubleshooting output stays stable and isn't part of the user-facing contract.
26. As a user, I want unsupported/foreign system languages (e.g. French, Japanese) to fall back to English, so that the UI never renders keys or blanks.

## Implementation Decisions

- **Locales**: exactly three — `en` (source), `zh-Hans`, `zh-Hant` (script subtags, 港澳台用词 in one catalog). English is the source language: source strings are written in English first; the two Chinese catalogs are translations of the English source.
- **Frontend i18n library**: `vue-i18n` (Vue 3 standard), installed as a new dependency, initialized at app bootstrap with message catalogs for the three locales. Message keys are English camelCase identifiers; no Chinese as key text.
- **Locale resolution order**: explicit user choice > system detection > English fallback. System detection is a **manual normalization** of `navigator.language` (values differ by OS/browser: `zh-CN`, `zh-TW`, `zh-HK`, `zh-Hans`, `zh-Hant`, `zh-Hans-CN`, `zh-Hant-TW`, sometimes underscored `zh_CN`, so they never match the catalog keys directly): contains `Hant` or region `TW`/`HK`/`MO` → `zh-Hant`; any other `zh*` → `zh-Hans`; everything else → `en`. The same normalization applies to the `languagechange` event.
- **Persistence**: the choice (including the "follow system" sentinel) is stored in `localStorage` and read at startup; while the sentinel is active the app re-resolves on launch and reacts to the browser `languagechange` event. No backend store is involved.
- **Language selector**: a new row in the settings panel with options labeled in their own language (`English` / `简体中文` / `繁體中文（港澳台）`) plus `跟随系统`; switching updates the i18n locale and `localStorage` immediately. Implemented as a custom popover sharing the `DropdownMenu` component with the format dropdown — the native `<select>` option popup is rendered by WebKit and cannot be styled (white-on-white in dark mode).
- **Rust message catalog**: a pure function mapping `(key, locale)` → rendered message, covering three groups: (a) conversion-error user-facing variants, (b) command-layer validation/拒绝 strings, (c) system notification summary/body. Tables for all three locales live next to each other so parity is testable.
- **Rust locale state**: process-wide locale set via a new `set_locale` invoke; the frontend calls it at mount and on every language change. Default before the first call is `en` (commands and notifications only occur after user interaction, so the frontend always wins in practice). The interface shape does not change: commands still return already-rendered strings, and the conversion-completed event still carries a rendered `errorMessage`.
- **Locale rendering convention** (Rust): validation/拒绝 errors capture the locale once at the command boundary and thread it as an explicit parameter (pure, parallel-testable); event payloads and system notifications render via `current()` at send time, so a language switch mid-batch is reflected immediately.
- **`set_locale` push**: failures are retried once, then logged via plugin-log — the frontend locale has already applied, so a backend miss only delays alignment until the next switch.
- **Internal failure details never interpolate**: technical details inside conversion errors (e.g. HEIC buffer/plane failures) are dedicated catalog entries (`error.heic_processing_failed` via `ConversionError::HeicProcessingFailed`), not free-form strings dropped into `{format}` placeholders, so no English fragments leak into Chinese sentences.
- **App-provided dialog strings are ours**: the native dialog's OS chrome stays out of scope, but strings the app passes to it (filter name, dialog title) are localized like any other UI copy.
- **Rendered-at-source errors**: because `user_message` and command errors are rendered with the current Rust locale, the frontend displays them directly (tooltip / dialog suffix) and only localizes its own surrounding text (dialog prefix, labels).
- **No scope for logs**: Rust `log` records and frontend `plugin-log` calls keep their current wording (Chinese) and are explicitly not part of the localized surface.
- **Dynamic content**: placeholders for filenames, paths, counts and sizes remain interpolated after translation; `formatSize` units stay English (`KB`/`MB`) and are not localized.

## Testing Decisions

- A good test exercises external behavior: given a locale, a key renders a non-empty string in the expected language; switching locale changes what a mounted component renders; catalogs are structurally complete. Tests never assert on internal store of the i18n machinery.
- **Frontend catalog parity test**: all three locale objects expose identical key sets, no values are empty; prior art — existing unit tests under `src/__tests__/unit/`.
- **Pinned-locale component/store tests**: existing tests that assert Chinese copy are pinned to `zh-Hans` through a shared test helper (set the i18n locale before mounting), so assertions stay deterministic on any host language; new tests may pin `en` to also exercise source strings.
- **Language-switch test**: selecting a language in the settings panel updates the rendered text, persists the sentinel/value to `localStorage`, and triggers the `set_locale` invoke (Rust side covered separately).
- **Locale-normalization test**: given representative `navigator.language` values (`zh-CN`, `zh-TW`, `zh-HK`, `zh-Hans`, `zh-Hant`, `zh-Hant-TW`, `zh_CN`, `zh-Hans-CN`, `en-US`, `fr-FR`), the resolver maps each to the expected catalog key (`zh-Hans` / `zh-Hant` / `en`).
- **Rust catalog parity test**: the `(key, locale)` tables for the three locales have identical key sets and no empty values; per-locale rendering assertions use the pure function (no shared state, safe under parallel test execution).
- **Rust locale round-trip**: one test sets the process locale and observes it at a command boundary inside `with_current` (a global lock serializes shared-state observation; the restored value is captured under the lock so post-restore assertions cannot race). The system-notification send path itself is not unit-testable; its per-locale rendering is covered by pure `message()` assertions instead.
- Prior art on the Rust side: existing integration tests under `src-tauri/tests/` (tempfile-driven, assert user-visible strings).

## Out of Scope

- Translating logs (Rust log records, frontend plugin-log output) — troubleshooting output stays as-is.
- Documentation translation (README, docs, CHANGELOG, in-repo markdown).
- Number/date/unit localization (`formatSize` keeps English units; no `Intl.NumberFormat` overhaul).
- RTL languages and locales beyond the three (architecture permits adding later; no UI work now).
- Native file-dialog chrome (OS-drawn window, buttons) — strings the app passes in (filter name, title) are localized; see Implementation Decisions.
- Persisting the locale in backend settings (localStorage only).
- Localization of third-party component-library internals beyond what vue-i18n covers.

## Further Notes

- String inventory at spec time: ~150 frontend occurrences of Chinese copy (templates + store alert strings), ~20 Rust user-facing strings (error-variant messages, command validation rejections, notification title/body), and ~15 existing test assertions on Chinese copy.
- English source strings do not exist yet and must be written as part of implementation ("源串待增加"); they are the authoritative wording the Chinese translations follow.
- Adding a locale later = one frontend catalog entry + one Rust table + a detection-rule line, nothing else.
- `vue-i18n` is a new runtime dependency; it is the only dependency this feature adds.
- Test-only seams that live in production code (`setTestLocale`, `ConversionError::user_message_with`, `locale::with_current`) were accepted at review as the smaller evil — they are thin wrappers over the same catalog the runtime uses, not parallel implementations.
- Review outcome: both axes (Standards / Spec) confirmed after one fix round; deviations from this spec found and fixed along the way include the shared `DropdownMenu` extraction, the `validate_convert_request` extraction, dead-variant removal (`UnsupportedInputFormat`), and the outside-click test requiring `attachTo: document.body`.
