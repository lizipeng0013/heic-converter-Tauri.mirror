# HEIC 格式转换器 (Tauri 版)

![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?logo=tauri)
![Vue](https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vue.js)
![License](https://img.shields.io/badge/License-GPL--3.0-blue.svg)
![Version](https://img.shields.io/badge/Version-0.4.3-blue)

一款基于 **Tauri 2** 和 **Vue 3** 构建的轻量级跨平台 HEIC 图片转换工具。它可以将 HEIC/HEIF 格式的高清图片快速转换为通用的 JPG 或 PNG 格式。

### ✨ v0.4.0 更新亮点

本版本聚焦**桌面深度集成**和**性能极致优化**，添加了系统托盘、系统通知、关闭保护等桌面级功能，并重写了 JPEG 编码引擎，转换性能大幅提升。

- 🔔 **系统通知**：转换完成后窗口最小化时发送系统通知；Linux 下点击通知可打开窗口。
- 🖥️ **系统托盘**：完整托盘支持，右键菜单可控制窗口显示/退出，托盘图标实时展示转换状态。
- 🛡️ **关闭保护**：转换中关闭窗口弹出确认对话框；有待转换文件时提示确认；托盘退出时检查转换状态。
- ⚡ **JPEG 编码性能飞跃**：将 JPEG 编码器从 `image` crate 替换为 `turbojpeg`，编码速度提升 30-50%。
- 📜 **虚拟滚动**：引入 `@tanstack/vue-virtual`，文件列表支持虚拟滚动，流畅处理数千个文件。
- ❌ **失败标签页**：新增独立的转换失败文件标签页，方便集中处理异常。
- 🧹 **优雅停止**：使用任务队列实现真正的可控停止，停止后转换即时中断，无残留任务。
- 🎨 **UI 全面进化**：无边现代风格、SVG 自适应标题栏（高度可配置）、ScrollArea 优化滚动体验。
- 🐧 **Deepin/UOS 适配**：只产出 DSG / Debian 两种 deb 安装包（移除全部 .exe/.dmg/.AppImage 打包目标）；运行时依赖由 `dpkg-shlibdeps` 自动解析（cargo-deb `$auto`），仅手动收紧 libheif1 下限并补托盘库，定制桌面入口模板。
- 🔧 **大规模重构**：窗口/托盘操作迁移至 Tauri 2 前端 API；mimalloc 全局内存分配器；原子操作替代 Mutex 减少锁竞争。

---

### 🌟 特性

- 🚀 **完全本地运行**：所有转换逻辑均在您的本地设备上执行，不会上传任何图片到云端，隐私绝对安全。
- ⚡ **Rust 原生高性能**：采用 Rust 原生实现，支持并行批量处理，转换速度极快，无内存限制。
- 🖱️ **拖拽即用**：支持批量拖拽文件，界面简洁直观，操作流畅。
- 📦 **多格式支持**：支持转换为 JPG、PNG、WebP、BMP、TIFF、ICO 六种主流格式。
- 🎛️ **自定义质量**：JPEG 格式支持 0-100 质量调节，灵活控制画质与文件大小。
- ⏸️ **转换控制**：支持停止和继续转换，提供更灵活的批量处理体验。
- 📁 **智能文件管理**：双标签页设计，状态分组显示，支持一键定位转换文件。
- 🎨 **现代化界面**：基于 Shadcn-vue 构建的精致 UI，支持深色模式，视觉体验舒适。
- 📦 **跨平台内核**：代码基于 Tauri 2，理论上可在 Windows / macOS / Linux 编译；**本项目目前只提供 deepin/UOS 的 deb 安装包**。

---

### 🏗️ 技术栈与架构

本项目严格遵循现代前端工程化标准构建。

#### 核心框架

- **Core**: Tauri 2.0 (Rust Backend)
- **Frontend**: Vue 3 (Composition API + TypeScript)
- **Build**: Vite
- **Styling**: Shadcn-vue (Zinc Theme) + Tailwind CSS
- **Converter**: libheif-rs (Rust Native) + turbojpeg (JPEG 编码) + image (通用编码)
- **Parallel**: rayon (Rust Parallel Processing)
- **Memory**: mimalloc (全局内存分配器)
- **Tray**: Tauri 2 内置 tray-icon
- **Notification**: notify-rust / tauri-plugin-notification

#### 项目结构 (v0.4.0)

```
src/                          # 前端代码
├── assets/                   # 静态资源
├── components/               # 通用组件
│   ├── dialog/              # 对话框组件 (关闭确认等)
│   ├── layout/              # 布局相关组件 (如 TitleBar)
│   └── ui/                  # UI 组件库 (Button, Card, Dialog, ScrollArea 等)
├── views/                    # 页面级组件
│   ├── MainView.vue         # 主视图
│   ├── file/                # 文件相关视图
│   ├── settings/            # 设置面板
│   └── status/              # 状态栏
├── stores/                   # 状态管理
│   └── conversionStore.ts   # 转换业务状态
├── utils/                    # 工具函数
├── types/                    # TypeScript 类型定义
└── App.vue                   # 应用根组件

src-tauri/                    # Rust 后端代码
├── heic-converter-dsg.desktop    # DSG 变体桌面入口（绝对路径 Exec）
├── heic-converter-debian.desktop # Debian 变体桌面入口
├── info                 # DSG 打包源文件 (info)
├── src/
│   ├── commands/            # Tauri 命令
│   │   └── conversion.rs    # 转换命令 (窗口命令已迁移至前端 API)
│   ├── converters/          # 转换器模块
│   │   ├── dispatcher.rs    # 转换调度器
│   │   ├── heic.rs          # HEIC 专用转换器
│   │   ├── generic.rs       # 通用转换器
│   │   └── common.rs        # 通用工具
│   ├── services/            # 业务服务
│   │   └── conversion.rs    # 转换服务 (任务队列实现优雅停止)
│   ├── setup/               # Tauri 插件初始化
│   └── utils/               # 工具函数
└── Cargo.toml               # Rust 依赖配置
```

**架构优势**：

- **解耦**：`MainView` 负责布局，不处理业务；`Stores` 负责状态，不处理 UI；`Services` 负责逻辑，不关心框架。
- **高性能**：Rust 原生实现，支持并行处理，无内存限制，转换速度极快。
- **可测试性**：独立的 Services 和 Utils 函数非常容易编写单元测试。
- **可扩展性**：模块化设计，易于添加新的转换格式和功能。

---

### ⚡ 性能与实现

本项目采用 **Rust 原生实现**，提供卓越的转换性能和稳定性。

- **技术实现**：使用 Rust 的 `libheif-rs` 库进行原生 HEIC 解码，结合 `turbojpeg` 进行高速 JPEG 编码，`image` 库支持多格式输出，完全脱离浏览器 Wasm 限制。
- **并行处理**：利用 `rayon` 库实现批量转换的并行处理，充分利用多核 CPU 性能，大幅提升批量转换速度。
- **无内存限制**：原生 Rust 实现，不受浏览器内存沙箱限制，可稳定处理大量高清图片。
- **全局内存分配器**：采用 `mimalloc` 替代系统默认分配器，减少内存碎片，提升整体性能。
- **JPEG 编码优化**：使用 `turbojpeg` 替代 `image` 内置编码器，JPEG 编码速度提升 30-50%。
- **锁优化**：使用原子操作替代 Mutex，减少多线程下的锁竞争和上下文切换开销。
- **性能表现**：单张图片转换速度极快，批量并行处理效率极高，适合处理任意数量的图片。
- **优化配置**：Release 版本启用 LTO（链接时优化）、代码生成单元化、strip 瘦身等高级优化，确保最佳性能。

---

### 📦 安装与运行

#### 环境要求

| 工具 | 版本 | 来源 |
|------|------|------|
| Node.js | 24.21.0 | **无需自行安装**：`pnpm install` 会按 `package.json` 的 `devEngines.runtime` 把它装成项目级依赖 |
| pnpm | 12.8.1 | 需独立安装（不依赖 Node.js）；版本由 `packageManager` 字段锁定，装别的版本会被自动纠正 |
| Rust | 1.98.1 | 由仓库根的 `rust-toolchain.toml` 锁定，rustup 自动安装该版本 |
| cargo-deb | 3.8.0 | **仅打包时需要**，见下文第 5 步 |

Node 装在 `node_modules/.bin/node`，**不在 PATH 里**。所有前端命令都通过 `pnpm` 执行
（`pnpm tauri dev`、`pnpm test`、`pnpm exec tsc` 等）；确需直接调用 node 时用
`./node_modules/.bin/node`。`@types/node` 已对齐 Node 24。

#### 1. 克隆项目

```bash
git clone https://atomgit.com/hotime/heic-converter.git
cd heic-converter
```

#### 2. 安装依赖

```bash
pnpm install
```

这一步会一并把 Node 24.21.0 装进 `node_modules/.bin/`。

#### 3. 开发模式运行

此命令将启动 Vite 开发服务器和 Tauri 窗口。

```bash
pnpm tauri dev
```

#### 4. 构建二进制

```bash
pnpm tauri build
```

**只编译二进制主程序，不产出任何安装包。** `.exe` / `.dmg` / `.AppImage` 等打包目标已全部
移除（`src-tauri/tauri.conf.json` 中 `bundle.active = false`），产物是单个可执行文件：

```
src-tauri/target/release/heic-converter
```

需要安装包请走下一步。

#### 5. deb 打包（DSG / Debian）

tauri-bundler 的安装路径（`/usr/bin`、`/usr/share/applications` 等）是硬编码的，无法产出纯 DSG
布局，因此**本项目的打包路径是 cargo-deb**：`tauri build` 只负责产二进制，deb 元数据全部写在
`src-tauri/Cargo.toml` 的 `[package.metadata.deb]`（`dsg` 与 `debian` 两个变体）。

前置条件：deepin/UOS 仓库没有 `cargo-deb` 包，需先装一次（3.8.0，锁 `--locked` 保证依赖可复现）：

```bash
cargo install cargo-deb --version 3.8.0 --locked
```

装好后执行打包脚本：

```bash
pnpm deb          # 一次构建两种包（CI 用）：tauri build 一次 + dsg + debian
pnpm deb:dsg      # UOS/DSG 规范：仅 /opt/apps/top.hotime.heic-converter/ 布局
pnpm deb:debian   # 标准 Debian 布局：/usr/bin + 桌面入口 + hicolor 图标
```

每个脚本都是 `tauri build`（产二进制）→ `cargo deb`（打包装配）→ `fixup-dsg-deb.sh`（仅 DSG 变体重排
copyright 与 changelog）。

- DSG 安装布局：`/opt/apps/top.hotime.heic-converter/`（`info` + `entries/applications` + `entries/icons` + `files/bin`），无 postinst 钩子。
- 两个变体的主程序二进制均命名为 `heic-converter`（DSG 位于 `/opt/apps/{appid}/files/bin/` 内无命名冲突；Debian 变体与包名一致）。
- DSG 源文件：`src-tauri/info`（appid 为倒置域名，**上架前必须换成已拥有的域名**）。两个变体的桌面入口分别由 `heic-converter-dsg.desktop`（DSG，绝对路径 `Exec`）与 `heic-converter-debian.desktop`（Debian）生成，图标为 scalable SVG（`src/assets/app-icon.svg`），两个 deb 变体直接打包该源文件，无需多档 PNG。
- 两个变体包名分别为 `top.hotime.heic-converter`（DSG，反转域名）与 `heic-converter`（Debian），产物在 `src-tauri/target/debian/`。
- 运行时依赖由 `dpkg-shlibdeps` 经 cargo-deb `$auto` 自动解析（GTK/WebKit 栈、`libheif1` 等，带正确版本下限），仅手写 `$auto` 扫不到的托盘库 `libayatana-appindicator3-1`。
- `copyright` 与 `changelog.gz` 都落在 `usr/share/doc/<pkg>/`；DSG 变体由 `src-tauri/scripts/fixup-dsg-deb.sh` 重打包后搬到 `entries/doc/<appid>/`，包内无 `/usr` 残留（脚本会在 `/usr` 有残留时报错退出）。
- changelog 由 `assets` 声明为 `usr/share/doc/<pkg>/changelog`（**不带 `.gz`**），cargo-deb 的 `compressed_assets()` 会自动 gzip 成 `changelog.gz`（Debian 政策 §12.7 要求压缩格式）。查看：`zcat /opt/apps/top.hotime.heic-converter/entries/doc/*/changelog.gz`。
  - 不要改用 `[metadata.deb] 的 changelog = ` 字段：那条路径会固定生成 `changelog.Debian.gz`，语义是「Debian 打包侧 changelog」，用于与上游 changelog 并存的双文件场景；**我们自身就是上游，只有一份 changelog，该场景不成立**。
- 运行时依赖里 `libheif1` 的版本下限交给 `depends` 的 `$auto` 自动解析，**不要手写**：libheif1 不做符号版本化（无 `verdef` 节，符号全挂 `@Base`），`dpkg-shlibdeps` 会改为逐符号查 `libheif1:amd64.symbols` 取引入版本的最大值；实测本二进制用到的 20 个 `heif_*` 符号最高为 `heif_init`/`heif_deinit`（1.13.0 引入），所以 `$auto` 给出的 1.13.0 就是真实下限。`libheif-rs` 的 `v1_18` 只是编译期 FFI gate，不构成运行时依赖 —— 手写 `libheif1 (>= 1.18)` 只会与 `$auto` 产出重复。
- 已知缺口：包内没有 `md5sums`（§12.7 要求）。cargo-deb 不生成、fixup 脚本也不补 —— 补它得让 DSG 与 Debian 两个变体都走一遍后处理，收益不抵复杂度。deepin 软件商店安装不校验它，先这样。
- 手写依赖只有 `libayatana-appindicator3-1`：`libappindicator-sys` 是纯 dlopen，不在 ELF 的 `NEEDED` 里，`$auto` 扫不到，漏了装完托盘图标不显示。

##### bump version 时必须同步 changelog

`debian/changelog` 是**发版记录的唯一来源**：CNB Release 正文由 `ci/gen-release-body.sh` 从中提取（取 tag 对应版本块，`  * ` 转 `- `），无需在别处重复写一遍。所以**每次 bump version 都要在 `debian/changelog` 顶部补一个版本块**：

```
heic-converter (0.4.2) unstable; urgency=medium

  * 一条改动摘要。

 -- hotime <xiaoyqde@126.com>  Wed, 30 Sep 2026 10:00:00 +0800
```

- 块首行格式固定为 `heic-converter (<版本号>) unstable; urgency=medium`，缩进两格写 `  * ` 条目。
- 末行**必须**是 ` -- 维护者 <邮箱>  RFC822 日期`，否则 `dpkg-parsechangelog` 解析失败。
- 改完用下面两条命令自检（缺任一步都会导致 tag 发布时 Release 正文为空）：

```bash
dpkg-parsechangelog -l debian/changelog          # 格式校验，必须能解析出版本/日期/维护者
CNB_BRANCH=v0.4.2 sh ci/gen-release-body.sh     # 预览 Release 正文，应命中而非走兜底
```

> changelog 里**不要写 `#` 注释**，`dpkg-parsechangelog` 不接受；本节这类约定说明放 README。

---

### 🧪 测试与代码检查

以下命令与 CI 门禁（`.cnb.yml`）完全一致，在 amd64 / arm64 上行为相同：

| 命令 | 作用 | CI 阶段 |
|------|------|----------|
| `pnpm exec vue-tsc --noEmit` | TypeScript 类型检查，含 `.vue`（无输出即通过） | PR + push 门禁 |
| `pnpm lint:rust` | `cargo clippy --all-targets --all-features -- -D warnings`，警告即失败 | PR + push 门禁 |
| `pnpm test` | Vitest 单元/组件测试（`vitest run`） | PR + push 门禁 |
| `pnpm build` | `vue-tsc --noEmit && vite build`，打包前置（`pnpm deb` → `tauri build`） | push / tag 打包 |
| `pnpm test:watch` | Vitest watch 模式，本地开发用 | — |
| `pnpm test:coverage` | 覆盖率报告（`@vitest/coverage-v8`） | — |
| `pnpm test:e2e` | Playwright 端到端测试 | — |
| `pnpm lint` | ESLint + `--fix` 自动修 | 仅本地 |
| `pnpm format` | Prettier 格式化 `src/` | 仅本地 |
| `pnpm format:rust:check` | `cargo fmt --check` | 仅本地 |
| `cargo test --manifest-path src-tauri/Cargo.toml` | Rust 单元测试 | PR + push 门禁 |

Rust 侧改动后建议至少跑一次：

```bash
pnpm lint:rust
cargo test --manifest-path src-tauri/Cargo.toml
```

这些脚本都定义在 `package.json` 的 `scripts` 里，是 CI 的唯一入口 —— 改脚本名时记得同步
`.cnb.yml`，否则流水线会静默跳过对应检查。

---

### 📖 使用指南

#### 1. 选择格式与质量

在右侧设置面板中，选择目标格式：

- **JPG / JPEG**：最常用的图片格式，支持压缩，适合日常使用和分享
- **PNG**：无损压缩格式，适合需要保留透明度或高质量的场景
- **WebP**：现代图片格式，压缩率更高，文件更小
- **BMP**：无压缩位图格式，兼容性好
- **TIFF**：专业图像格式，适合印刷和出版
- **ICO**：图标格式，适合制作网站图标

JPG 和 WebP 格式下可调整图片质量（0-100），数值越高画质越好，文件体积也越大。

#### 2. 添加文件

支持两种方式添加文件：

- **拖拽上传**：直接将 HEIC 文件拖入左侧文件列表区域，支持批量拖拽。
- **点击选择**：点击底部的"选择文件"按钮，从文件管理器中选取。

文件列表支持双标签页切换：

- **全部**：显示所有文件
- **转换中/已完成/失败**：按状态分组显示，方便管理

#### 3. 开始转换

点击右下侧的"开始批量转换"按钮。

- 列表中的文件状态会实时更新（等待 -> 转换中 -> 完成）。
- 进度条和状态标签会清晰展示当前处理进度。
- 转换过程中可点击"停止"按钮暂停转换，点击"继续"恢复转换。

#### 4. 查看结果

转换完成后，文件将自动保存到您的下载目录（或您在设置中指定的输出目录）。

- 点击文件卡片右侧的"打开输出目录"按钮，可直接定位到转换文件所在位置。
- 文件列表支持一键定位功能，方便快速找到转换后的图片。

---

### 🔮 未来规划

- [ ] **自动监听文件夹**：监听系统文件夹变化，自动拉入新文件进行转换。
- [ ] **全局快捷键**：支持添加快捷键和自定义热键。
- [ ] **批量重命名**：支持转换时批量重命名文件。
- [ ] **更多压缩选项**：支持更多精细的压缩参数控制。
- [ ] **图片预览**：在转换前预览原图效果。

---

### 📄 开源协议

本项目采用 GPL-3.0 协议开源。

---

### 🤖 AI 辅助声明

本项目的 v0.3.0 版本重构、架构设计优化、UI 组件迁移以及技术文档编写，v0.4.0 的托盘、通知、关闭保护等桌面集成功能，以及性能优化工作，使用了 iflow + GLM-4.7 进行辅助生成。

**注意**: 本工具仅用于个人学习和合法用途，请勿用于侵犯他人版权的图片转换。
