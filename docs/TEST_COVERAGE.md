# 测试覆盖率报告

> 生成时间: 2026-08-22
> 状态: 由于网络限制，实际测试尚未运行，以下为测试框架统计

## 测试覆盖范围

### Rust 后端测试

| 模块 | 文件 | 测试用例数 | 覆盖内容 |
|------|------|------------|----------|
| 格式转换 | `common_test.rs` | 15 | OutputFormat、保存功能、错误处理 |
| HEIC 转换 | `heic_test.rs` | 12 | 格式检测、扩展名验证 |
| 分发器 | `dispatcher_test.rs` | 5 | 格式分发、路径构建 |
| 转换服务 | `conversion_test.rs` | 8 | 格式验证、错误处理 |
| 路径工具 | `path_test.rs` | 15 | 路径构建、目录验证 |
| **总计** | **5 个文件** | **55 个测试用例** | - |

### 前端测试

| 模块 | 文件 | 测试用例数 | 覆盖内容 |
|------|------|------------|----------|
| Store | `conversionStore.test.ts` | 18 | 文件管理、状态转换、设置 |
| Button 组件 | `Button.test.ts` | 8 | 渲染、变体、事件 |
| TitleBar 组件 | `TitleBar.test.ts` | 11 | 窗口操作、主题切换 |
| 工具函数 | `index.test.ts` | 7 | cn 函数、类名合并 |
| **总计** | **4 个文件** | **44 个测试用例** | - |

### E2E 测试

| 文件 | 测试场景数 | 覆盖内容 |
|------|------------|----------|
| `app.test.ts` | 10 | 应用加载、界面元素 |
| `file-operations.test.ts` | 4 | 文件操作、状态显示 |
| **总计** | **14 个测试场景** | - |

## 测试框架统计

```
总测试用例数: 113 个
├── Rust 后端: 55 个
├── 前端单元测试: 44 个
└── E2E 测试: 14 个场景

测试文件数: 11 个
├── Rust: 5 个
├── 前端: 4 个
└── E2E: 2 个

配置和文档: 6 个
├── vitest.config.ts
├── playwright.config.ts
├── setup.ts
├── TESTING.md
├── CI_CD.md
└── ci.yml
```

## 依赖添加

### Rust (Cargo.toml)
```toml
[dev-dependencies]
tempfile = "3.10"
rstest = "0.21"
```

###前端 (package.json)
```json
{
  "devDependencies": {
    "vitest": "^1.2.0",
    "@vue/test-utils": "^2.4.0",
    "jsdom": "^24.0.0",
    "@vitest/coverage-v8": "^1.2.0",
    "playwright": "^1.44.0"
  }
}
```

## 待完成项

由于网络限制（无法访问 rsproxy.cn 镜像），以下测试需要在网络恢复后执行：

1. **Rust 测试运行**
   ```bash
   cargo test --manifest-path src-tauri/Cargo.toml
   ```

2. **前端测试运行**
   ```bash
   pnpm install
   pnpm test:coverage
   ```

3. **E2E 测试运行**
   ```bash
   pnpm install
   npx playwright install
   pnpm test:e2e
   ```

## 预期覆盖率目标

| 层级 | 目标覆盖率 | 当前状态 |
|------|------------|----------|
| Rust 后端 | > 80% | 🔄 待运行 |
| 前端 | > 70% | 🔄 待运行 |

## 下一步

1. 恢复网络连接
2. 运行所有测试
3. 根据测试结果调整测试用例
4. 生成覆盖率报告
5. 确保所有测试通过

## 测试命令速查

```bash
# Rust
cargo test --manifest-path src-tauri/Cargo.toml

# 前端
pnpm test              # 运行所有测试
pnpm test:coverage     # 运行测试并生成覆盖率
pnpm test:unit         # 仅运行单元测试
pnpm test:e2e          # 运行 E2E 测试

# Lint
pnpm lint              # ESLint
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings

# 格式检查
pnpm format --check
cargo fmt --all --manifest-path src-tauri/Cargo.toml -- --check
```
