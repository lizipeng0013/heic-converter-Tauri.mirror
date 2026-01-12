# HEIC 格式转换器 (Tauri 版)

![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?logo=tauri)
![Vue](https://img.shields.io/badge/Vue-3.4-4FC08D?logo=vue.js)
![License](https://img.shields.io/badge/License-MIT-green.svg)

一款基于 **Tauri 2** 和 **Vue 3** 构建的轻量级跨平台 HEIC 图片转换工具。它可以将 HEIC/HEIF 格式的高清图片快速转换为通用的 JPG 或 PNG 格式。

### ✨ v0.2.0 重大更新

本版本对代码架构进行了全面重构，并引入了现代化的 UI 框架。

- 🚀 **TypeScript 全面迁移**：核心代码已完全由 JavaScript 迁移至 TypeScript，利用静态类型检查，提升代码健壮性与开发体验。
- 📦 **模块化架构重构**：遵循 Vue 3 Composition API 最佳实践，将业务逻辑、状态管理、UI 组件解耦为 `Services`、`Stores`、`Views` 等独立模块，极大提升可维护性。
- 🎨 **现代化 UI 升级**：完全重构用户界面，引入 [Shadcn-vue](https://www.shadcn-vue.com/) 组件库。界面不仅美观专业，且配色基于 Zinc 体系，更加和谐统一。
- 🌓 **深色模式支持**：新增一键切换深色/浅色主题功能，适应不同光照环境，保护视力。
- 🪟 **自定义标题栏**：放弃原生 OS 窗口边框，采用 Tauri API 构建自定义应用标题栏，实现沉浸式桌面应用体验。
- 🏗️ **架构预留**：清晰的前后端分层设计，为未来无缝切换至 Rust 后端（libheif）奠定了坚实基础。

---

### 🌟 特性

- 🚀 **完全本地运行**：所有转换逻辑均在您的本地设备上执行，不会上传任何图片到云端，隐私绝对安全。
- 🖱️ **拖拽即用**：支持批量拖拽文件，界面简洁直观，操作流畅。
- 🎨 **现代化界面**：基于 Shadcn-vue 构建的精致 UI，支持深色模式，视觉体验舒适。
- 📦 **跨平台**：支持 Windows, macOS 和 Linux (Tauri 的强大特性)。
- 🔄 **批量处理**：一次性添加多个文件，一键批量转换，状态实时反馈。

---

### 🏗️ 技术栈与架构

本项目严格遵循现代前端工程化标准构建。

#### 核心框架

- **Core**: Tauri 2.0 (Rust Backend)
- **Frontend**: Vue 3 (Composition API + TypeScript)
- **Build**: Vite
- **Styling**: Shadcn-vue (Zinc Theme) + Tailwind CSS
- **Converter**: heic2any (WebAssembly)

#### 项目结构 (v0.2.0)

```
src/
├── assets/              # 静态资源
├── components/          # 通用组件
│   ├── common/         # 全局通用组件
│   └── layout/         # 布局相关组件 (如 CustomTitleBar)
├── views/              # 页面级组件 (MainView, FileListArea, etc.)
├── composables/        # 组合式函数
├── stores/             # 状态管理
│   └── conversion.ts # 核心业务状态
├── services/           # 业务逻辑层
│   └── conversionService.ts # 转换逻辑实现 (预留后端切换接口)
├── utils/              # 工具函数
├── types/              # TypeScript 类型定义
└── App.vue             # 应用根组件
```

**架构优势**：

- **解耦**：`MainView` 负责布局，不处理业务；`Stores` 负责状态，不处理 UI；`Services` 负责逻辑，不关心框架。
- **可测试性**：独立的 Services 和 Utils 函数非常容易编写单元测试。
- **可扩展性**：未来如果需要将 `heic2any` 替换为 Rust 后端，只需修改 `ConversionService`，UI 层面完全不需要改动。

---

### ⚠️ 性能与实现

本项目目前采用 **前端 Wasm 方案** 实现转换功能。

- **技术实现**：使用 `heic2any` 库将 C++ 的 `libheif` 编译为 WebAssembly，在浏览器/Tauri 视图中运行。
- **适用场景**：非常适合转换 10-50 张以内的日常照片。
- **内存限制**：由于受限于浏览器的内存沙箱机制（WASM 运行在 V8 中），不建议一次性处理几百张超大高清图，可能会导致内存溢出（OOM）。
- **性能表现**：单张图片转换速度极快，接近原生速度。

---

### 📦 安装与运行

#### 环境要求

- **Node.js**: v16.0 或更高版本
- **包管理器**: pnpm (推荐), npm 或 yarn
- **Rust**: 1.70+ (如果需要编译 Tauri 后端)

#### 1. 克隆项目

```bash
git clone https://github.com/your-username/your-heic-converter.git
cd your-heic-converter
```

#### 2. 安装依赖

```bash
pnpm install
```

#### 3. 开发模式运行

此命令将启动 Vite 开发服务器和 Tauri 窗口。

```bash
pnpm tauri dev
```

#### 4. 构建生产版本

编译打包生成适用于您操作系统的可执行文件（.exe, .dmg, .AppImage 等）。

```bash
pnpm tauri build
```

构建完成后，可执行文件位于 `src-tauri/target/release/bundle/` 目录下。

---

### 📖 使用指南

#### 1. 选择格式与质量

在右侧设置面板中，选择目标格式（JPG / JPEG 或 PNG）。JPG 格式下可调整图片质量（0-100），数值越高画质越好，文件体积也越大。

#### 2. 添加文件

支持两种方式添加文件：

- **拖拽上传**：直接将 HEIC 文件拖入左侧文件列表区域。
- **点击选择**：点击底部的“选择文件”按钮，从文件管理器中选取。

#### 3. 开始转换

点击右下侧的“开始批量转换”按钮。

- 列表中的文件状态会实时更新（等待 -> 转换中 -> 完成）。
- 进度条和状态标签会清晰展示当前处理进度。

#### 4. 查看结果

转换完成后，文件将自动保存到您的下载目录（或您在设置中指定的输出目录）。您可以在文件管理器中直接查看转换后的图片。

---

### 🔮 未来规划

- [ ] **引入 Rust 后端**：使用原生 `libheif` 库替代前端 Wasm，支持多线程处理海量文件。
- [ ] **更多输出格式**：增加 WEBP 等现代格式支持。
- [ ] **自定义输出质量**：支持更精细的压缩参数控制。
- [ ] **自动监听文件夹**：监听系统文件夹变化，自动拉入新文件进行转换。
- [ ] **全局快捷键**：支持添加快捷键和自定义热键。

---

### 📄 开源协议

本项目采用 MIT 协议开源。

---

### 🤖 AI 辅助声明

本项目的 v0.2.0 版本重构、架构设计优化、UI 组件迁移以及技术文档编写过程中，使用了 GLM-4.7 进行辅助生成。开发者基于 AI 提供的原型和代码建议，结合实际的 Tauri 2 与 Vue 3 环境进行了细致的调试、适配与优化，确保了应用的稳定性与代码的高质量。

**注意**: 本工具仅用于个人学习和合法用途，请勿用于侵犯他人版权的图片转换。
