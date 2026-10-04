# 01: i18n 基础设施与语言选择

**What to build:** 作为用户，我在设置面板看到语言选择行（English / 简体中文 / 繁體中文（港澳台）/ 跟随系统），切换后界面立即以该语言渲染种子文案，重启后仍然生效；未手动选择时跟随系统语言（经手动归一化：含 Hant 或地区 TW/HK/MO → zh-Hant，其余 zh* → zh-Hans，其他 → en），系统语言变化实时跟随。Rust 侧在前端挂载与每次切换时收到 locale（默认 en），错误与通知目录骨架就位、三语键集一致。

**Blocked by:** None (can start immediately)

**Status:** done

- [x] vue-i18n 装配完成，三语目录（en / zh-Hans / zh-Hant）键集一致性与非空值 parity 测试通过
- [x] 归一化解析器对代表性系统值（zh-CN、zh-TW、zh-HK、zh-Hans、zh-Hant、zh-Hant-TW、zh_CN、zh-Hans-CN、en-US、fr-FR）映射正确，有测试
- [x] localStorage 持久化：手动选择与"跟随系统"哨兵存取正确；哨兵激活时监听 languagechange 实时重解析
- [x] 设置面板语言选择行可用：切换后已接入的种子文案立即变化，选择写入 localStorage 并触发 Rust set_locale invoke
- [x] Rust 侧：set_locale 命令写入进程级 locale，默认 en；纯函数消息目录骨架（三语表并列）+ 键集 parity 测试 + 一轮次 round-trip 测试
- [x] 共享测试 helper：钉住指定 locale 渲染组件/store，测试结果与宿主机语言无关
- [x] 全量既有测试保持通过（vue-tsc、eslint 干净）
