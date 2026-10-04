# 测试指南

本文档描述了如何运行 HEIC 转换器的测试套件。

## 前置条件

- Node.js 18+
- pnpm
- Rust 1.70+

## 安装依赖

```bash
# 安装前端依赖
pnpm install

# 安装 Rust 依赖
cd src-tauri && cargo check
```

## 运行测试

### Rust 后端测试

```bash
# 运行所有测试
cargo test --manifest-path src-tauri/Cargo.toml

# 运行特定模块的测试
cargo test --manifest-path src-tauri/Cargo.toml converters::common_test

# 运行带输出的测试
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture

# 生成覆盖率报告
cargo tarpaulin --manifest-path src-tauri/Cargo.toml --out Html
```

### 前端测试

```bash
# 运行单元测试
pnpm test

# 运行测试并监听文件变化
pnpm test:watch

# 运行测试并生成覆盖率报告
pnpm test:coverage

# 运行单元测试
pnpm test:unit
```

### E2E 测试

```bash
# 安装 Playwright 浏览器
pnpm exec playwright install

# 运行 E2E 测试
pnpm test:e2e

# 运行特定测试
pnpm exec playwright test src/__tests__/e2e/app.test.ts

# 运行带 UI 的测试
pnpm exec playwright test --headed
```

## 测试结构

```
src-tauri/tests/           # Rust 后端测试
├── capabilities_test.rs  # 能力（capability）配置断言测试
├── commands_test.rs      # 命令层（输入校验/输出目录确认）测试
├── converters/           # 转换器测试
│   ├── common_test.rs    # 通用格式转换测试
│   ├── heic_test.rs      # HEIC 专用转换器测试
│   └── dispatcher_test.rs # 格式分发器测试
├── services/             # 服务层测试
│   └── conversion_test.rs # 转换服务测试
├── utils/                # 工具函数测试
│   ├── path_test.rs      # 路径工具测试
│   └── locale_test.rs    # 三语消息目录与 locale 状态测试
└── mod.rs               # 测试入口

src/__tests__/            # 前端测试
├── unit/                 # 单元测试
│   ├── stores/          # Pinia Store 测试
│   │   └── conversionStore.test.ts
│   ├── components/      # 组件测试
│   │   ├── FileCard.test.ts
│   │   ├── ui/         # UI 组件测试
│   │   │   └── Button.test.ts
│   │   └── layout/     # 布局组件测试
│   │       └── TitleBar.test.ts
│   ├── views/          # 视图测试
│   │   ├── FileListArea.test.ts
│   │   ├── MainView.test.ts
│   │   ├── SettingsPanel.test.ts
│   │   └── StatusBar.test.ts
│   ├── i18n/            # 多语言测试
│   │   ├── normalize.test.ts   # 系统标签归一化
│   │   ├── catalog.test.ts     # 目录键集/非空 parity
│   │   └── locale.test.ts      # 持久化与 Rust 下发
│   └── utils/          # 工具函数测试
│       └── index.test.ts
├── e2e/                  # E2E 测试
│   ├── app.test.ts       # 应用基础测试
│   └── file-operations.test.ts
└── setup.ts             # 测试配置
```

## 测试覆盖范围

### Rust 后端

| 模块 | 测试内容 | 状态 |
|------|----------|------|
| OutputFormat | 格式解析、扩展名、质量参数 | ✅ 已测试 |
| save_image_buffer | JPEG/PNG/WebP/BMP/TIFF/ICO 保存 | ✅ 已测试 |
| ConversionError | 错误类型、用户友好消息 | ✅ 已测试 |
| is_heic_format | HEIC 检测、扩展名验证 | ✅ 已测试 |
| build_target_path | 路径构建、文件名处理 | ✅ 已测试 |
| validate_output_folder | 目录验证、权限检查（创建即失败探针） | ✅ 已测试 |
| plan_target_paths / reserve_unique_output_path | 输出路径规划、加序号兜底、符号链接占用跳过、批量上限 | ✅ 已测试 |
| validate_input_paths | 输入校验：普通文件、扩展名白名单、批量数量上限 | ✅ 已测试 |
| check_dimensions / check_file_size | 解码前尺寸与文件大小限额 | ✅ 已测试 |
| confirm_output_folder / ensure_output_confirmed | 输出目录确认闸（未确认拒绝转换）、错误文案按 locale 渲染 | ✅ 已测试 |
| capabilities/default.json | 权限收敛断言（敏感权限已移除、stat scope 精确） | ✅ 已测试 |
| utils/locale | 三语目录（en/zh-Hans/zh-Hant）键集与非空 parity、message/message_fmt 占位符替换、with_current 锁内往返（串行化并恢复）、通知文案三语 | ✅ 已测试 |
| ConversionError::user_message | 按 locale 渲染（三语互异、参数替换生效） | ✅ 已测试 |
| 校验文案渲染 | validate_input_paths / validate_output_folder / validate_convert_request（未选文件 En/ZhHans）/ 命令拒绝在 En 与 ZhHans 下分别渲染对应语言 | ✅ 已测试 |

### 前端

| 模块 | 测试内容 | 状态 |
|------|----------|------|
| conversionStore | 文件添加、状态转换、设置更新、clearErrors、输出目录确认、转换失败弹窗语言文案（钉 zh-Hans/en） | ✅ 已测试 |
| Button | 渲染、变体、点击事件 | ✅ 已测试 |
| TitleBar | 窗口操作、主题切换、事件发射 | ✅ 已测试 |
| utils | cn 函数、类名合并 | ✅ 已测试 |
| FileListArea | 清空按钮转换中禁用、清空失败只清失败列表、tab 切换 | ✅ 已测试 |
| FileCard | 已完成项显示"源文件名 → 生成文件名"（含无输出路径回退） | ✅ 已测试 |
| i18n | normalizeLocale 系统标签归一化、目录键集/非空 parity、locale 持久化与 Rust 下发（set_locale 命令）、组件文案渲染（钉 zh-Hans，兜底 en） | ✅ 已测试 |
| SettingsPanel / DropdownMenu | 语言下拉切换、更多格式下拉开合与选择（共享 DropdownMenu 组件）、点击外部关闭 | ✅ 已测试 |

## 覆盖率目标

- Rust 后端: > 80%
- 前端: > 70%

## CI/CD

测试会自动在以下情况下运行：

1. 推送到 main 分支
2. 创建 Pull Request
3. 发布新版本

详见 `.atomcode/workflows/ci.yml` 配置。

## 故障排除

### Rust 测试失败

1. 确保依赖已安装: `cargo check`
2. 检查网络连接（下载依赖）
3. 清理构建缓存: `cargo clean`

### 前端测试失败

1. 确保依赖已安装: `pnpm install`
2. 检查 Vitest 配置: `vitest.config.ts`
3. 清理缓存: `rm -rf node_modules && pnpm install`

### E2E 测试失败

1. 确保应用已构建: `pnpm tauri build`
2. 安装浏览器: `pnpm exec playwright install`
3. 检查端口是否被占用: `lsof -i :1420`

## 编写新测试

### Rust 测试

在 `src-tauri/tests/` 目录下创建新的测试文件：

```rust
use heic_converter_lib::converters::common::OutputFormat;

#[test]
fn test_my_feature() {
    let format = OutputFormat::from_str("jpg", 90).unwrap();
    assert!(matches!(format, OutputFormat::Jpeg(90)));
}
```

### 前端测试

在 `src/__tests__/unit/` 目录下创建新的测试文件：

```typescript
import { describe, it, expect } from 'vitest'
import { myFunction } from '@/utils/myFunction'

describe('myFunction', () => {
  it('should do something', () => {
    const result = myFunction('test')
    expect(result).toBe('expected')
  })
})
```

## 参考资源

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [Vitest Documentation](https://vitest.dev/)
- [Vue Test Utils](https://test-utils.vuejs.org/)
- [Playwright Documentation](https://playwright.dev/)
