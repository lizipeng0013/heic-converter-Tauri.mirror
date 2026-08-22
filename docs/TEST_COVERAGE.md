# 测试覆盖率报告

> 生成时间: 2026-08-22
> 状态: 测试框架已就绪，覆盖率报告需本地生成

## 快速生成覆盖率报告

### 方法 1: 使用自动化脚本

```bash
./generate-coverage.sh
```

### 方法 2: 手动生成

#### Rust 后端

```bash
# 安装 cargo-tarpaulin（仅需一次）
cargo install cargo-tarpaulin

# 生成 HTML 覆盖率报告
cd src-tauri
cargo tarpaulin --out Html --output-dir ../target/coverage

# 查看报告
xdg-open target/coverage/tarpaulin-report.html
```

#### 前端

```bash
# 生成覆盖率报告
pnpm test:coverage

# 查看报告
xdg-open coverage/index.html
```

## 测试覆盖范围

### Rust 后端测试 (39 个测试)

| 模块 | 测试数 | 覆盖内容 |
|------|--------|----------|
| converters/common | 15 | 格式解析、保存功能、错误处理 |
| converters/heic | 12 | HEIC 检测、格式验证 |
| converters/dispatcher | 5 | 格式分发、路径构建 |
| services/conversion | 8 | 格式验证、错误处理 |
| utils/path | 15 | 路径构建、目录验证 |

### 前端测试 (6 个测试)

| 模块 | 测试数 | 覆盖内容 |
|------|--------|----------|
| stores/conversionStore | 1 | Pinia Store 测试 |
| components/ui/Button | 1 | 按钮组件测试 |
| components/layout/TitleBar | 13 | 标题栏组件测试 |

**总计: 45 个测试用例**

## 覆盖率目标

| 层级 | 目标覆盖率 | 当前状态 |
|------|------------|----------|
| Rust 后端 | > 80% | 🔄 待本地生成 |
| 前端 | > 70% | 🔄 待本地生成 |

## 基准测试

### 运行基准测试

```bash
cd src-tauri
cargo bench
```

### 基准测试内容

1. **JPEG 编码性能**
   - 不同质量设置 (60, 80, 90, 100)
   - 不同图像尺寸 (640x480, 1280x720, 1920x1080, 3840x2160)

2. **输出格式性能**
   - JPEG, PNG, WebP, BMP

3. **路径构建性能**
   - 不同长度的路径

### 基准测试结果

基准测试结果会生成在 `target/criterion/` 目录中。

## 优化建议

### Rust 后端

1. **添加更多边界条件测试**
   - 大文件处理
   - 内存限制测试
   - 并发压力测试

2. **性能优化**
   - 使用 criterion 进行基准测试
   - 监控关键路径性能
   - 优化内存使用

3. **错误处理测试**
   - 模拟各种错误场景
   - 验证错误恢复机制

### 前端

1. **提升测试覆盖率**
   - 添加更多组件测试
   - 测试 Store 的所有方法
   - 测试工具函数

2. **E2E 测试**
   - 配置 Playwright
   - 测试完整用户流程
   - 跨浏览器测试

## 参考资源

- [cargo-tarpaulin 文档](https://github.com/xd009642/tarpaulin)
- [Vitest 覆盖率](https://vitest.dev/guide/coverage.html)
- [criterion 基准测试](https://bheisler.github.io/criterion.rs/book/index.html)
