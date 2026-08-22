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
npx playwright install

# 运行 E2E 测试
pnpm test:e2e

# 运行特定测试
npx playwright test src/__tests__/e2e/app.test.ts

# 运行带 UI 的测试
npx playwright test --headed
```

## 测试结构

```
src-tauri/tests/           # Rust 后端测试
├── converters/           # 转换器测试
│   ├── common_test.rs    # 通用格式转换测试
│   ├── heic_test.rs      # HEIC 专用转换器测试
│   └── dispatcher_test.rs # 格式分发器测试
├── services/             # 服务层测试
│   └── conversion_test.rs # 转换服务测试
├── utils/                # 工具函数测试
│   └── path_test.rs      # 路径工具测试
└── mod.rs               # 测试入口

src/__tests__/            # 前端测试
├── unit/                 # 单元测试
│   ├── stores/          # Pinia Store 测试
│   │   └── conversionStore.test.ts
│   ├── components/      # 组件测试
│   │   ├── ui/         # UI 组件测试
│   │   │   └── Button.test.ts
│   │   └── layout/     # 布局组件测试
│   │       └── TitleBar.test.ts
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
| validate_output_folder | 目录验证、权限检查 | ✅ 已测试 |

### 前端

| 模块 | 测试内容 | 状态 |
|------|----------|------|
| conversionStore | 文件添加、状态转换、设置更新 | ✅ 已测试 |
| Button | 渲染、变体、点击事件 | ✅ 已测试 |
| TitleBar | 窗口操作、主题切换、事件发射 | ✅ 已测试 |
| utils | cn 函数、类名合并 | ✅ 已测试 |

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
2. 安装浏览器: `npx playwright install`
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
