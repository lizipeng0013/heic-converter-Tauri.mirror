# CI/CD 工作流

本文档描述了 HEIC 转换器的 AtomGit Action CI/CD 配置。

## 工作流概览

| 工作流 | 触发条件 | 主要任务 |
|--------|----------|----------|
| CI | Push/PR | 测试、检查、构建 |
| Release | Tag | 发布、打包 |

## CI 工作流

`.gitcode/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

concurrency:
  max: 1
  exceed-action: QUEUE

permissions:
  repository: read
  pr: write

env:
  CARGO_TERM_COLOR: always
  NODE_VERSION: '18'

jobs:
  test:
    name: 测试
    runs-on: [ubuntu-latest, x64, small]
    steps:
      - name: 检出代码
        uses: checkout
      
      - name: 设置 Rust
        uses: setup-rust
        with:
          rust-version: stable
          components: rustfmt, clippy
      
      - name: 设置 Node.js
        uses: setup-node
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'pnpm'
      
      - name: 安装前端依赖
        run: pnpm install
      
      - name: 运行 Rust 测试
        run: cargo test --manifest-path src-tauri/Cargo.toml
      
      - name: 运行前端测试
        run: pnpm test:coverage
      
      - name: 运行 Rust Linter
        run: cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
      
      - name: 运行 ESLint
        run: pnpm lint
      
      - name: 检查代码格式
        run: |
          pnpm format --check
          cargo fmt --all --manifest-path src-tauri/Cargo.toml -- --check
      
      - name: 构建应用
        run: pnpm tauri build
      
      - name: 上传覆盖率报告
        uses: upload-artifact
        with:
          name: coverage-report
          path: coverage/
          retention-days: 7

  post-process:
    name: 后处理
    runs-on: [ubuntu-latest, x64, small]
    if: ${{ always() }}
    needs: test
    steps:
      - name: 发送通知
        run: |
          echo "## 工作流执行完成" >> $ATOMGIT_STEP_SUMMARY
          echo "| 状态 | 值 |" >> $ATOMGIT_STEP_SUMMARY
          echo "|------|----|" >> $ATOMGIT_STEP_SUMMARY
          echo "| 测试状态 | ${{ job.status }} |" >> $ATOMGIT_STEP_SUMMARY
```

## Release 工作流

`.gitcode/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  repository: write
  contents: read

env:
  NODE_VERSION: '18'

jobs:
  release:
    name: 发布
    runs-on: [ubuntu-latest, x64, medium]
    steps:
      - name: 检出代码
        uses: checkout
      
      - name: 设置 Rust
        uses: setup-rust
        with:
          rust-version: stable
      
      - name: 设置 Node.js
        uses: setup-node
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'pnpm'
      
      - name: 安装前端依赖
        run: pnpm install
      
      - name: 构建应用
        run: pnpm tauri build
      
      - name: 上传构建产物
        uses: upload-artifact
        with:
          name: heic-converter-${{ atomgit.ref_name }}
          path: src-tauri/target/release/bundle/*/*
          retention-days: 30
      
      - name: 创建 Release
        env:
          ATOMGIT_TOKEN: ${{ secrets.ATOMGIT_TOKEN }}
        run: |
          TAG_NAME="${{ atomgit.ref_name }}"
          curl -X POST \
            "${{ atomgit.api_url }}/repos/${{ atomgit.repository }}/releases" \
            -H "Authorization: token $ATOMGIT_TOKEN" \
            -H "Content-Type: application/json" \
            -d '{
              "tag_name": "'"$TAG_NAME"'",
              "name": "HEIC Converter '"$TAG_NAME"'",
              "body": "HEIC 格式转换器 v'"$TAG_NAME"'",
              "draft": false,
              "prerelease": false
            }'

  post-process:
    name: 发布后处理
    runs-on: [ubuntu-latest, x64, small]
    if: ${{ always() }}
    needs: release
    steps:
      - name: 发布完成通知
        run: |
          echo "## 发布流程完成" >> $ATOMGIT_STEP_SUMMARY
          echo "| 项目 | 值 |" >> $ATOMGIT_STEP_SUMMARY
          echo "|------|----|" >> $ATOMGIT_STEP_SUMMARY
          echo "| 版本 | ${{ atomgit.ref_name }} |" >> $ATOMGIT_STEP_SUMMARY
          echo "| 状态 | ${{ job.status }} |" >> $ATOMGIT_STEP_SUMMARY
```

## 配置说明

### 工作流文件位置

AtomGit Action 的 workflow 文件存放目录为：

```
.gitcode/workflows/<workflow-name>.yml
```

### Runner 标签体系

AtomGit 托管资源池使用**三段式标签**格式：`{os-version},{arch},{flavor}`

常用标签：
- `[ubuntu-latest, x64, small]` - Ubuntu 24.04 / x64 / 2核8G
- `[ubuntu-latest, x64, medium]` - Ubuntu 24.04 / x64 / 4核16G

### 预装工具

**构建工具**：Make, CMake, Maven, Gradle, npm, pip, yarn, **pnpm 8.x**

**语言运行时**：
- Node.js: 18, 20, 22, 24
- Rust: latest stable (via rustup)
- Python: 3.10, 3.11, 3.12
- Go: 1.21, 1.22, 1.23

### 上下文变量

| 变量 | 说明 |
|------|------|
| `atomgit.ref` | 触发分支或标签引用 |
| `atomgit.sha` | 触发提交的 SHA |
| `atomgit.repository` | 仓库全名 |
| `atomgit.event_name` | 触发事件类型 |
| `atomgit.actor` | 触发者用户名 |

### 权限控制

```yaml
permissions:
  repository: read  # 仓库读取权限
  pr: write         # PR 写入权限
  issue: write      # Issue 写入权限
```

快捷语法：
- `read-all`：所有权限设为 read
- `write-all`：所有权限设为 write
- `permissions: {}`：所有权限设为 none

### 并发控制

```yaml
concurrency:
  max: 1              # 最大并发数
  exceed-action: QUEUE # 超出时的策略：QUEUE（排队）或 IGNORE（忽略）
```

### 制品管理

```yaml
- name: 上传制品
  uses: upload-artifact
  with:
    name: artifact-name
    path: path/to/artifact
    retention-days: 7
```

## 本地验证

在推送之前，可以在本地运行完整的 CI 检查：

```bash
# Rust 测试
cargo test --manifest-path src-tauri/Cargo.toml

# 前端测试
pnpm test:coverage

# Lint
pnpm lint
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings

# 格式化检查
pnpm format --check
cargo fmt --all --manifest-path src-tauri/Cargo.toml -- --check

# 构建
pnpm tauri build
```

## 故障排除

### 测试失败

1. 检查测试输出日志
2. 确保所有依赖已安装
3. 在本地重现问题

### 构建失败

1. 检查 Rust 版本: `rustc --version`
2. 检查 Node.js 版本: `node --version`
3. 清理构建缓存: `cargo clean && rm -rf node_modules`

### 覆盖率报告

覆盖率报告会生成在以下位置：

- Rust: `target/debug/coverage/`
- 前端: `coverage/`

## 参考资源

- [AtomGit Action 文档](https://docs.atomgit.com/docs/help/home/org_project/pipeline/overview)
- [Runner 镜像与预装工具](https://docs.atomgit.com/docs/help/home/org_project/pipeline/syntax-reference/runner-images-tools)
- [Node.js 项目 CI 示例](https://docs.atomgit.com/docs/help/home/org_project/pipeline/examples/nodejs-ci)
