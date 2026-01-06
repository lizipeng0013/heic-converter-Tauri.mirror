# HEIC 格式转换器 (Tauri 版)

![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?logo=tauri)
![Vue](https://img.shields.io/badge/Vue-3.4-4FC08D?logo=vue.js)
![License](https://img.shields.io/badge/License-MIT-green.svg)

一款基于 **Tauri 2** 和 **Vue 3** 构建的轻量级跨平台 HEIC 图片转换工具。它可以将 HEIC/HEIF 格式的高清图片快速转换为通用的 JPG 或 PNG 格式。

## ✨ 特点

*   🚀 **完全本地运行**：所有转换逻辑均在您的本地设备上执行，不会上传任何图片到云端，隐私绝对安全。
*   🖱️ **拖拽即用**：支持批量拖拽文件，界面简洁直观。
*   🎨 **现代化 UI**：基于 Vue 3 Composition API 开发，拥有流畅的深色模式界面。
*   📦 **跨平台**：支持 Windows, macOS 和 Linux (Tauri 的强大特性)。
*   🔄 **批量处理**：一次性添加多个文件，一键批量转换。

## ⚠️ 性能与架构说明

本项目目前采用 **前端 Wasm 方案** 实现转换功能。

*   **技术实现**：使用 `heic2any` 库将 C++ 的 `libheif` 编译为 WebAssembly 在浏览器/Tauri 视图中运行。
*   **适用场景**：非常适合转换 **10-50 张** 以内的日常照片。
*   **内存限制**：由于受限于浏览器的内存沙箱机制，不建议一次性处理几百张超大高清图，可能会导致内存溢出（OOM）。
*   **性能表现**：单张图片转换速度极快，接近原生速度。

## 🛠️ 技术栈

*   **Core**: Tauri 2.0 (Rust Backend)
*   **Frontend**: Vue 3 (Script Setup) + Vite
*   **Styling**: Scoped CSS (No Framework)
*   **Converter**: heic2any (WebAssembly)

## 📦 安装与运行

### 环境要求

*   **Node.js**: v16.0 或更高版本
*   **包管理器**: pnpm (推荐), npm 或 yarn
*   **Rust**: 1.70+ (如果需要编译 Tauri 后端)

### 1. 克隆项目

```bash
git clone https://github.com/your-username/your-heic-converter.git
cd your-heic-converter
```

### 2. 安装依赖

```bash
pnpm install
```

### 3. 开发模式运行

此命令将启动 Vite 开发服务器和 Tauri 窗口。

```bash
pnpm tauri dev
```

### 4. 构建生产版本

编译打包生成适用于您操作系统的可执行文件（`.exe`, `.dmg`, `.AppImage` 等）。

```bash
pnpm tauri build
```

构建完成后，可执行文件位于 `src-tauri/target/release/bundle/` 目录下。

## 📖 使用指南

1.  **选择格式**：在左侧面板选择输出格式（JPG 或 PNG）。
2.  **添加文件**：点击上传区域或直接将 HEIC 图片拖拽到框中。
3.  **开始转换**：点击“开始批量转换”按钮。
4.  **下载结果**：转换完成后，点击文件右侧的下载按钮保存图片。

## 🔮 未来规划

*   [ ] 引入 Rust 原生 `libheif` 后端，支持多线程处理海量文件。
*   [ ] 添加更多输出格式支持（如 WEBP）。
*   [ ] 支持自定义输出画质。
*   [ ] 自动监听文件夹变化。

## 📄 开源协议

本项目采用 [MIT](LICENSE) 协议开源。

## 🤖 AI 辅助声明 

本项目的初始代码架构、Vue 组件核心逻辑以及技术文档编写过程中，使用了 **GLM-4.7** 进行辅助生成。开发者基于 AI 提供的原型和代码建议，结合实际的 Tauri 2 环境进行了调试、优化和适配，使其能够成功运行。 

---

**注意**: 本工具仅用于个人学习和合法用途，请勿用于侵犯他人版权的图片转换。

